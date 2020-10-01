// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Target - struct that collects draw calls to be used in a renderer

use crate::color::Color;
use crate::font::Font;
use crate::image::TextureFilter;
use crate::image::TextureWrap;
use crate::math::Matrix4;
use crate::math::Transform;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::renderer::Light;
use crate::storage::Builtins;
use crate::storage::Handle;

pub struct Target<'a, 'b> {
    // global
    pub clear_color: Color,
    pub skybox: bool,
    pub transform: Transform,
    pub(crate) builtins: &'b Builtins,

    // meshes
    pub shader: Option<&'a Handle<Shader>>,
    pub material: Option<&'a Handle<Material>>,
    pub texture_filter: TextureFilter,
    pub texture_wrap: TextureWrap,
    pub texture_mipmaps: bool,
    pub(crate) mesh_orders: Vec<OrdersByShader>,

    // shadows & lights
    pub shadow_bias: f32,
    pub shadow_cascades: [f32; 4],
    pub shadows: bool,
    pub lights: [Light; 4],

    // lines
    pub line_color: Color,
    pub(crate) line_orders: Vec<LineOrder>,

    // shapes
    pub shape_color: Color,
    pub(crate) shape_orders: Vec<ShapeOrder>,

    // text
    pub font_size: u32,
    pub font: Option<&'a Handle<Font>>,
    pub text_color: Color,
    pub(crate) text_orders: Vec<TextOrder>,
}

pub(crate) struct OrdersByShader {
    pub(crate) shader: Handle<Shader>,
    pub(crate) orders: Vec<OrdersByMaterial>,
}

pub(crate) struct OrdersByMaterial {
    pub(crate) material: Handle<Material>,
    pub(crate) orders: Vec<MeshOrder>,
}

pub(crate) struct MeshOrder {
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) local_to_world: Matrix4,
    pub(crate) shadows: bool,
    pub(crate) sampler_index: u32,
}

pub(crate) struct TextOrder {
    pub(crate) size: u32,
    pub(crate) color: Color,
    pub(crate) font: Handle<Font>,
    pub(crate) text: String,
    pub(crate) transform: Transform,
}

pub(crate) struct LineOrder {
    pub(crate) color: Color,
    pub(crate) points: [Vector3; 2],
    pub(crate) transform: Transform,
}

pub(crate) struct ShapeOrder {
    pub(crate) color: Color,
    pub(crate) points: [Vector3; 3],
    pub(crate) transform: Transform,
}

impl<'b> Target<'_, 'b> {
    pub(crate) fn new(builtins: &'b Builtins) -> Self {
        Self {
            mesh_orders: vec![],
            text_orders: vec![],
            line_orders: vec![],
            shape_orders: vec![],
            clear_color: Color::WHITE,
            text_color: Color::BLACK,
            line_color: Color::BLACK,
            shape_color: Color::BLACK,
            transform: Transform::default(),
            lights: [
                Light::directional((-1.0, -1.0, 1.0), Color::WHITE, true),
                Light::NONE,
                Light::NONE,
                Light::NONE,
            ],
            texture_filter: TextureFilter::Linear,
            texture_wrap: TextureWrap::Repeat,
            font_size: 24,
            font: None,
            shader: None,
            material: None,
            texture_mipmaps: true,
            skybox: false,
            shadow_cascades: [0.1, 0.25, 0.7, 1.0],
            shadow_bias: 0.002,
            shadows: true,
            builtins,
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Handle<Mesh>) {
        let default_shader = if self.shadows {
            &self.builtins.phong_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_mesh_wireframe(&mut self, mesh: &Handle<Mesh>) {
        let shader = self.builtins.wireframe_shader.clone();
        let material = self.builtins.white_material.clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                shadows: false,
                sampler_index: 0,
            },
        );
    }

    pub fn draw_cube(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.phong_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.cube_mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_sphere(&mut self) {
        let default_shader = if self.shadows {
            &self.builtins.phong_shader
        } else {
            &self.builtins.unshaded_shader
        };
        let shader = self.shader.unwrap_or(default_shader).clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.sphere_mesh.clone(),
                local_to_world: self.transform.as_matrix(),
                shadows: self.shadows,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_surface(&mut self) {
        let shader = self
            .shader
            .unwrap_or(&self.builtins.unshaded_shader)
            .clone();
        let material = self
            .material
            .unwrap_or(&self.builtins.white_material)
            .clone();

        self.add_mesh_order(
            material,
            shader,
            MeshOrder {
                mesh: self.builtins.surface_mesh.clone(),
                local_to_world: Transform::positioned(0.0, 0.0, 0.0).as_matrix(),
                shadows: false,
                sampler_index: self.sampler_index(),
            },
        );
    }

    pub fn draw_grid(&mut self) {
        let size = 100;
        let half = size / 2;
        let width = 1.0;

        // TODO: replace with push/pop
        let temp_color = self.line_color;

        for x in -half..half {
            let xx = x as f32 * width;
            let z_min = -half as f32 * width;
            let z_max = half as f32 * width;

            // set color
            self.line_color = match x {
                0 => Color::rgba(0, 0, 255, 150),
                _ if x % 10 == 0 => Color::rgba(255, 255, 255, 150),
                _ => Color::rgba(255, 255, 255, 50),
            };

            self.draw_line((xx, 0.0, z_min), (xx, 0.0, z_max));
        }

        for z in -half..half {
            let zz = z as f32 * width;
            let x_min = -half as f32 * width;
            let x_max = half as f32 * width;

            // set color
            self.line_color = match z {
                0 => Color::rgba(255, 0, 0, 150),
                _ if z % 10 == 0 => Color::rgba(255, 255, 255, 150),
                _ => Color::rgba(255, 255, 255, 50),
            };

            self.draw_line((x_min, 0.0, zz), (x_max, 0.0, zz));
        }

        self.line_color = temp_color;
    }

    pub fn draw_text<T, V>(&mut self, text: T, position: V)
    where
        T: AsRef<str>,
        V: Into<Vector2>,
    {
        let font = self.font.unwrap_or(&self.builtins.fira_font).clone();

        let mut transform = self.transform;
        transform.position += position.into().extend(0.0);

        self.text_orders.push(TextOrder {
            size: self.font_size,
            color: self.text_color,
            text: text.as_ref().to_string(),
            transform,
            font,
        });
    }

    pub fn draw_line<V>(&mut self, point_1: V, point_2: V)
    where
        V: Into<Vector3>,
    {
        self.line_orders.push(LineOrder {
            color: self.line_color,
            points: [point_1.into(), point_2.into()],
            transform: self.transform,
        });
    }

    pub fn draw_shape(&mut self, points: &[Vector2]) {
        // don't draw polygon with less than 2 points
        if points.len() < 3 {
            return;
        }

        // triangulate points
        let first = points[0].extend(0.0);
        for i in 2..points.len() {
            self.shape_orders.push(ShapeOrder {
                color: self.shape_color,
                points: [first, points[i - 1].extend(0.0), points[i].extend(0.0)],
                transform: self.transform,
            });
        }
    }

    pub fn draw_rectangle<V>(&mut self, bottom_left: V, size: V)
    where
        V: Into<Vector2>,
    {
        let bl = bottom_left.into();
        let s = size.into();
        self.draw_shape(&[
            Vector2::new(bl.x, bl.y + s.y),
            bl + s,
            Vector2::new(bl.x + s.x, bl.y),
            bl,
        ]);
    }

    fn add_mesh_order(
        &mut self,
        material: Handle<Material>,
        shader: Handle<Shader>,
        order: MeshOrder,
    ) {
        match self.mesh_orders.iter_mut().find(|so| so.shader == shader) {
            Some(so) => match so.orders.iter_mut().find(|mo| mo.material == material) {
                Some(mo) => mo.orders.push(order),
                None => so.orders.push(OrdersByMaterial {
                    material,
                    orders: vec![order],
                }),
            },
            None => self.mesh_orders.push(OrdersByShader {
                shader,
                orders: vec![OrdersByMaterial {
                    material,
                    orders: vec![order],
                }],
            }),
        }
    }

    const fn sampler_index(&self) -> u32 {
        use TextureFilter as F;
        use TextureWrap as W;

        match (self.texture_filter, self.texture_wrap, self.texture_mipmaps) {
            (F::Linear, W::Repeat, true) => 0,
            (F::Linear, W::Repeat, false) => 1,
            (F::Linear, W::ClampBorder, true) => 2,
            (F::Linear, W::ClampBorder, false) => 3,
            (F::Linear, W::ClampEdge, true) => 4,
            (F::Linear, W::ClampEdge, false) => 5,
            (F::Nearest, W::Repeat, true) => 6,
            (F::Nearest, W::Repeat, false) => 7,
            (F::Nearest, W::ClampBorder, true) => 8,
            (F::Nearest, W::ClampBorder, false) => 9,
            (F::Nearest, W::ClampEdge, true) => 10,
            (F::Nearest, W::ClampEdge, false) => 11,
        }
    }
}
