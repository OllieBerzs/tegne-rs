// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::sync::Arc;
use std::time::Instant;

use super::Order;
use super::RenderStats;
use super::Target;
use crate::camera::Camera;
use crate::camera::CameraType;
use crate::color::colors;
use crate::device::Device;
use crate::device::IN_FLIGHT_FRAME_COUNT;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::pipeline::AttachmentType;
use crate::pipeline::Light;
use crate::pipeline::Material;
use crate::pipeline::PushConstants;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::pipeline::ShadowMapUniform;
use crate::pipeline::WorldData;
use crate::resource::Ref;

const CASCADE_SPLITS: [f32; 3] = [0.2, 0.4, 1.0];

pub(crate) struct ForwardRenderer {
    shadow_framebuffers: Vec<Vec<Framebuffer>>,
    shadow_uniforms: Vec<ShadowMapUniform>,
    shadow_shader: Shader,
    shadow_map_size: u32,
    start_time: Instant,
}

pub(crate) struct ForwardDrawOptions<'a> {
    pub(crate) framebuffer: &'a Framebuffer,
    pub(crate) shader_layout: &'a ShaderLayout,
    pub(crate) target: Target,
}

impl ForwardRenderer {
    pub(crate) fn new(device: &Arc<Device>, shader_layout: &ShaderLayout) -> Result<Self> {
        profile_scope!("new");

        let shadow_map_size = 2048;

        let mut shadow_framebuffers = vec![];
        let mut shadow_uniforms = vec![];
        for frame in 0..IN_FLIGHT_FRAME_COUNT {
            shadow_framebuffers.push(vec![]);
            for _ in 0..CASCADE_SPLITS.len() {
                shadow_framebuffers[frame].push(Framebuffer::new(
                    device,
                    shader_layout,
                    FramebufferOptions {
                        attachment_types: &[AttachmentType::Depth],
                        camera_type: CameraType::Orthographic,
                        multisampled: false,
                        width: shadow_map_size,
                        height: shadow_map_size,
                    },
                )?);
            }

            shadow_uniforms.push(ShadowMapUniform::new(
                shader_layout,
                [
                    shadow_framebuffers[frame][0].stored_view(),
                    shadow_framebuffers[frame][1].stored_view(),
                    shadow_framebuffers[frame][2].stored_view(),
                ],
            )?);
        }

        let shadow_shader = Shader::new(
            device,
            &shadow_framebuffers[0][0],
            shader_layout,
            include_bytes!("../../assets/shaders/shadow.shader"),
            ShaderOptions {
                front_cull: false,
                ..Default::default()
            },
        )?;

        Ok(Self {
            start_time: Instant::now(),
            shadow_framebuffers,
            shadow_uniforms,
            shadow_shader,
            shadow_map_size,
        })
    }

    pub(crate) fn draw(
        &self,
        device: &Device,
        options: ForwardDrawOptions<'_>,
    ) -> Result<RenderStats> {
        let framebuffer = options.framebuffer;
        let clear = options.target.clear();
        let cmd = device.command_buffer();

        let light_dir = Vector3::new(-1.0, -2.0, -1.0).unit();

        let mut light_matrices = [Matrix4::identity(); 4];
        let mut cascade_splits = [0.0; 4];

        // shadow mapping
        if options.target.do_shadow_mapping() {
            // bind other random shadow map set
            device.cmd_bind_descriptor(
                cmd,
                self.shadow_uniforms[(device.current_frame() + 1) % IN_FLIGHT_FRAME_COUNT]
                    .descriptor(),
                &options.shader_layout,
            );

            // render shadow map for each cascade
            for (i, cs) in CASCADE_SPLITS.iter().enumerate() {
                let shadow_framebuffer = &self.shadow_framebuffers[device.current_frame()][i];

                // frustum-fit light camera
                // get view frustum corners from NDC
                let mut view_cam = framebuffer.camera.clone();
                // view_cam.depth = 25.0; // TODO: configurable max depth
                view_cam.depth *= cs;
                let view_cam =
                    Camera::perspective(view_cam.width, view_cam.height, view_cam.depth, 90);

                let cam_inv = view_cam.matrix().inverse().expect("bad matrix");
                let mut corners = vec![];
                for x in &[-1.0, 1.0] {
                    for y in &[-1.0, 1.0] {
                        for z in &[0.0, 1.0] {
                            let corner = cam_inv * Vector4::new(*x, *y, *z, 1.0);
                            corners.push(corner.shrink() / corner.w);
                        }
                    }
                }

                // get bounding sphere radius
                // sphere makes it axis-aligned
                let corner_count = corners.len() as f32;
                let center: Vector3 = corners.iter().sum::<Vector3>() / corner_count;
                let r = corners.iter().map(|v| (center - *v).length()).sum::<f32>()
                    / corners.len() as f32;

                // stabilize shadow map by using texel units
                let texel_size = (r * 2.0) / self.shadow_map_size as f32;
                let light_pos = {
                    let p = center - light_dir * r;
                    (p / texel_size).floor() * texel_size
                };
                let size = {
                    let s = r * 2.0;
                    (s / texel_size).floor() * texel_size
                };

                // create depth camera
                let mut depth_cam = Camera::orthographic(size, size, size);
                depth_cam.transform.look_in_dir(light_dir, Vector3::up());
                depth_cam.transform.position = light_pos;

                shadow_framebuffer.world_uniform().update(WorldData {
                    lights: [Default::default(); 4],
                    world_matrix: depth_cam.matrix(),
                    camera_position: framebuffer.camera.transform.position,
                    time: self.start_time.elapsed().as_secs_f32(),
                    cascade_splits: [0.0; 4],
                    light_matrices: [Matrix4::identity(); 4],
                    bias: 0.0,
                })?;

                device.cmd_begin_render_pass(cmd, shadow_framebuffer, clear);
                self.setup_pass(device, shadow_framebuffer);
                self.bind_world(device, shadow_framebuffer, &options);
                device.cmd_bind_shader(cmd, &self.shadow_shader);
                for s_order in options.target.orders_by_shader() {
                    for m_order in s_order.orders_by_material() {
                        self.bind_material(device, m_order.material(), &options)?;
                        for order in m_order.orders() {
                            if order.cast_shadows {
                                self.draw_order(device, order, &options, &mut 0)?;
                            }
                        }
                    }
                }
                device.cmd_end_render_pass(cmd);

                // set uniform variables for normal render
                light_matrices[i] = depth_cam.matrix();
                cascade_splits[i] = cs * r;
            }

            // bind current shadow map set
            device.cmd_bind_descriptor(
                cmd,
                self.shadow_uniforms[device.current_frame()].descriptor(),
                &options.shader_layout,
            );
        }

        // normal render
        // setup lights
        let main_light = Light {
            coords: light_dir.extend(0.0),
            color: colors::WHITE.to_rgba_norm_vec(),
        };
        let other_lights = options.target.lights();

        // update world uniform
        framebuffer.world_uniform().update(WorldData {
            lights: [
                main_light,
                other_lights[0],
                other_lights[1],
                other_lights[2],
            ],
            world_matrix: framebuffer.camera.matrix(),
            camera_position: framebuffer.camera.transform.position,
            time: self.start_time.elapsed().as_secs_f32(),
            bias: options.target.bias(),
            cascade_splits,
            light_matrices,
        })?;

        device.cmd_begin_render_pass(cmd, framebuffer, clear);
        self.setup_pass(device, framebuffer);
        self.bind_world(device, framebuffer, &options);

        let mut drawn_indices = 0;
        let mut shaders_used = 0;
        let mut materials_used = 0;
        let mut draw_calls = 0;

        for s_order in options.target.orders_by_shader() {
            self.bind_shader(device, s_order.shader());
            shaders_used += 1;
            for m_order in s_order.orders_by_material() {
                self.bind_material(device, m_order.material(), &options)?;
                materials_used += 1;
                for order in m_order.orders() {
                    self.draw_order(device, order, &options, &mut drawn_indices)?;
                    draw_calls += 1;
                }
            }
        }

        device.cmd_end_render_pass(cmd);

        Ok(RenderStats {
            time: self.start_time.elapsed().as_secs_f32(),
            drawn_triangles: drawn_indices / 3,
            drawn_indices,
            shaders_used,
            materials_used,
            draw_calls,
        })
    }

    fn setup_pass(&self, device: &Device, framebuffer: &Framebuffer) {
        let cmd = device.command_buffer();
        device.cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        device.cmd_set_line_width(cmd, 1.0);
    }

    fn bind_world(
        &self,
        device: &Device,
        framebuffer: &Framebuffer,
        options: &ForwardDrawOptions<'_>,
    ) {
        let cmd = device.command_buffer();
        device.cmd_bind_descriptor(
            cmd,
            framebuffer.world_uniform().descriptor(),
            options.shader_layout,
        );
    }

    fn bind_shader(&self, device: &Device, shader: &Ref<Shader>) {
        let cmd = device.command_buffer();
        shader.with(|s| device.cmd_bind_shader(cmd, s));
    }

    fn bind_material(
        &self,
        device: &Device,
        material: &Ref<Material>,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        let descriptor = material.with(|m| m.descriptor())?;
        device.cmd_bind_descriptor(cmd, descriptor, options.shader_layout);
        Ok(())
    }

    fn draw_order(
        &self,
        device: &Device,
        order: Order,
        options: &ForwardDrawOptions<'_>,
        drawn_indices: &mut u32,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        let albedo_index = order.albedo.with(|t| t.image_index());
        let (vb, ib, n) = order
            .mesh
            .with(|m| (m.vertex_buffer(), m.index_buffer(), m.index_count()));

        if let Some(framebuffer) = order.framebuffer {
            let frame_descriptor = framebuffer.with(|f| f.descriptor());
            device.cmd_bind_descriptor(cmd, frame_descriptor, &options.shader_layout);
        }

        device.cmd_push_constants(
            cmd,
            PushConstants {
                model_matrix: order.model,
                sampler_index: order.sampler_index,
                albedo_index,
            },
            options.shader_layout,
        );
        device.cmd_bind_vertex_buffer(cmd, vb?);
        device.cmd_bind_index_buffer(cmd, ib?);
        device.cmd_draw(cmd, n);

        *drawn_indices += n;

        Ok(())
    }
}
