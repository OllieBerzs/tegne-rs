// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Context - draw-it application entrypoint

use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(feature = "hot-reload")]
use notify::DebouncedEvent;
#[cfg(feature = "hot-reload")]
use std::sync::mpsc;

use crate::camera::Camera;
use crate::camera::CameraType;
use crate::color::Color;
use crate::device::pick_gpu;
use crate::device::Device;
use crate::device::DeviceProperties;
use crate::error::Result;
use crate::image::Cubemap;
use crate::image::CubemapOptions;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::ImageFormat;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::instance::Instance;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::quality::Quality;
use crate::renderer::ForwardRenderer;
use crate::renderer::RenderStats;
use crate::renderer::Target;
use crate::resource::Builtins;
use crate::resource::Ref;
use crate::resource::ResourceManager;
use crate::surface::Surface;
use crate::surface::SurfaceProperties;
use crate::surface::Swapchain;
use crate::surface::WindowHandle;

#[cfg(feature = "ui")]
use crate::ui::Ui;

#[cfg(feature = "window")]
use crate::window::Window;

pub struct Context {
    // Renderers
    forward_renderer: ForwardRenderer,

    // UI
    #[cfg(feature = "ui")]
    ui: Option<Ui>,

    // Resources
    builtins: Builtins,
    resources: ResourceManager,
    skybox: Cubemap,

    // Vulkan
    window_framebuffers: Arc<Mutex<Vec<Framebuffer>>>,
    image_uniform: ImageUniform,
    shader_layout: Arc<ShaderLayout>,
    swapchain: Swapchain,
    device: Arc<Device>,
    surface: Surface,
    gpu_index: usize,
    instance: Arc<Instance>,

    // Misc
    pub main_camera: Camera,
    camera_type: CameraType,
    render_stats: RenderStats,
    render_stage: RenderStage,

    // Hot Reload
    #[cfg(feature = "hot-reload")]
    stop_senders: Vec<mpsc::Sender<DebouncedEvent>>,

    // Window
    #[cfg(feature = "window")]
    glfw: Option<glfw::Glfw>,
    #[cfg(feature = "window")]
    event_receiver: Option<std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>>,
}

#[derive(Debug, Copy, Clone)]
pub struct ContextOptions {
    pub quality: Quality,
    pub vsync: bool,
    pub camera: CameraType,
}

#[cfg(feature = "window")]
#[derive(Debug, Copy, Clone)]
pub struct WindowOptions<'title> {
    pub title: &'title str,
    pub width: u32,
    pub height: u32,
}

#[derive(Copy, Clone)]
enum RenderStage {
    Before,
    During,
}

impl Context {
    pub fn new(window: WindowHandle, options: ContextOptions) -> Result<Self> {
        profile_scope!("new");

        let instance = Arc::new(Instance::new()?);
        let surface = Surface::new(&instance, window)?;

        let quality = options.quality.options();

        // query GPU properties
        let mut surface_properties_list =
            SurfaceProperties::new(&instance, &surface, options.vsync)?;
        let mut device_properties_list = DeviceProperties::new(&instance, quality.msaa)?;

        // pick GPU
        let gpu_index = pick_gpu(&surface_properties_list, &device_properties_list)?;
        let surface_properties = surface_properties_list.remove(gpu_index);
        let device_properties = device_properties_list.remove(gpu_index);

        let device = Arc::new(Device::new(
            &instance,
            &surface_properties,
            device_properties,
            gpu_index,
        )?);

        let swapchain = Swapchain::new(&device, &surface, surface_properties)?;

        let shader_layout = ShaderLayout::new(&device)?;

        let mut image_uniform = ImageUniform::new(&device, &shader_layout, quality.anisotropy)?;

        let window_framebuffers =
            Framebuffer::for_swapchain(&device, &swapchain, &shader_layout, options.camera)?;

        let mut resources = ResourceManager::new();
        let builtins = Builtins::new(
            &device,
            &mut resources,
            &window_framebuffers[0],
            &shader_layout,
            &mut image_uniform,
        )?;

        let mut skybox = Cubemap::new(
            &device,
            CubemapOptions {
                format: ImageFormat::Rgba,
                top: &[255, 255, 255, 255],
                bottom: &[255, 255, 255, 255],
                front: &[255, 255, 255, 255],
                back: &[255, 255, 255, 255],
                left: &[255, 255, 255, 255],
                right: &[255, 255, 255, 255],
                size: 1,
            },
        )?;
        image_uniform.set_skybox(skybox.add_view()?);

        let forward_renderer = ForwardRenderer::new(
            &device,
            &shader_layout,
            &mut image_uniform,
            quality.shadow_map_size,
            quality.pcf,
        )?;

        let main_camera = Camera::new(
            options.camera,
            window.width as f32,
            window.height as f32,
            100.0,
        );

        Ok(Self {
            window_framebuffers: Arc::new(Mutex::new(window_framebuffers)),
            shader_layout: Arc::new(shader_layout),
            render_stage: RenderStage::Before,
            render_stats: Default::default(),
            camera_type: options.camera,
            skybox,
            forward_renderer,
            builtins,
            resources,
            image_uniform,
            swapchain,
            device,
            surface,
            gpu_index,
            instance,
            main_camera,
            #[cfg(feature = "ui")]
            ui: None,
            #[cfg(feature = "hot-reload")]
            stop_senders: vec![],
            #[cfg(feature = "window")]
            glfw: None,
            #[cfg(feature = "window")]
            event_receiver: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.device.wait_for_idle()?;
        self.surface.resize(width, height);
        self.main_camera.width = width as f32;
        self.main_camera.height = height as f32;
        self.swapchain
            .recreate(&self.instance, &self.surface, self.gpu_index)?;

        let mut framebuffers = self.window_framebuffers.lock().unwrap();
        *framebuffers = Framebuffer::for_swapchain(
            &self.device,
            &self.swapchain,
            &self.shader_layout,
            self.camera_type,
        )?;

        #[cfg(feature = "ui")]
        self.ui
            .as_mut()
            .unwrap()
            .resize(&mut self.image_uniform, width, height)?;

        Ok(())
    }

    pub fn draw_on_window(&mut self, draw_callback: impl Fn(&mut Target)) -> Result<()> {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw()?;
        }

        {
            let mut target = Target::new(&self.builtins)?;
            draw_callback(&mut target);

            #[cfg(feature = "ui")]
            {
                let ui = self.ui.as_ref().unwrap();
                if ui.drawn() {
                    target.blit_framebuffer(ui.framebuffer());
                }
            }

            let framebuffer =
                &mut self.window_framebuffers.lock().unwrap()[self.swapchain.current()];
            framebuffer.camera = self.main_camera.clone();

            self.render_stats +=
                self.forward_renderer
                    .draw(framebuffer, &self.shader_layout, target)?;
        }

        self.end_draw()?;

        Ok(())
    }

    pub fn draw(
        &mut self,
        framebuffer: &Ref<Framebuffer>,
        draw_callback: impl Fn(&mut Target),
    ) -> Result<()> {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw()?;
        }

        let mut target = Target::new(&self.builtins)?;
        draw_callback(&mut target);

        let stats =
            framebuffer.with(|f| self.forward_renderer.draw(f, &self.shader_layout, target))?;
        self.render_stats += stats;

        Ok(())
    }

    pub fn create_texture(&mut self, pixels: &[Color], width: u32) -> Result<Ref<Texture>> {
        let data = pixels
            .iter()
            .map(|p| vec![p.r, p.g, p.b, p.a])
            .flatten()
            .collect::<Vec<_>>();
        let texture = Texture::new(
            &self.device,
            &mut self.image_uniform,
            TextureOptions {
                format: ImageFormat::Rgba,
                data: &data,
                height: pixels.len() as u32 / width,
                width,
            },
        )?;
        Ok(self.resources.add_texture(texture))
    }

    pub fn set_skybox(&mut self, pixels: [&[u8]; 6], size: u32) -> Result<()> {
        let mut cubemap = Cubemap::new(
            &self.device,
            CubemapOptions {
                format: ImageFormat::Rgba,
                top: pixels[0],
                bottom: pixels[1],
                front: pixels[2],
                back: pixels[3],
                left: pixels[4],
                right: pixels[5],
                size,
            },
        )?;

        self.image_uniform.set_skybox(cubemap.add_view()?);
        self.skybox = cubemap;

        Ok(())
    }

    pub fn create_mesh(&mut self, options: MeshOptions<'_>) -> Result<Ref<Mesh>> {
        let mesh = Mesh::new(&self.device, options)?;
        Ok(self.resources.add_mesh(mesh))
    }

    pub fn combine_meshes(&mut self, meshes: &[Ref<Mesh>]) -> Result<Ref<Mesh>> {
        let mut offset = 0;
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut colors = vec![];
        for mesh in meshes {
            mesh.with(|m| {
                indices.extend(m.indices().iter().map(|i| i + offset));
                vertices.extend(m.vertices());
                normals.extend(m.normals());
                uvs.extend(m.uvs());
                colors.extend(m.colors());
                offset = vertices.len() as u32;
            });
        }

        let mesh = Mesh::new(
            &self.device,
            MeshOptions {
                vertices: &vertices,
                normals: &normals,
                uvs: &uvs,
                colors: &colors,
                indices: &indices,
            },
        )?;
        Ok(self.resources.add_mesh(mesh))
    }

    pub fn create_material(&mut self) -> Result<Ref<Material>> {
        let material = Material::new(&self.device, &self.shader_layout)?;
        Ok(self.resources.add_material(material))
    }

    pub fn create_framebuffer(
        &mut self,
        t: CameraType,
        width: u32,
        height: u32,
    ) -> Result<Ref<Framebuffer>> {
        let framebuffer = Framebuffer::new(
            &self.device,
            &self.shader_layout,
            &mut self.image_uniform,
            FramebufferOptions {
                attachment_formats: &[ImageFormat::Sbgra],
                camera_type: t,
                multisampled: self.device.is_msaa(),
                depth: true,
                width,
                height,
            },
        )?;
        Ok(self.resources.add_framebuffer(framebuffer))
    }

    pub fn resize_framebuffer(
        &mut self,
        framebuffer: &Ref<Framebuffer>,
        width: u32,
        height: u32,
    ) -> Result<()> {
        framebuffer.with(|f| f.resize(width, height, &mut self.image_uniform))
    }

    pub fn create_shader(&mut self, source: &[u8], options: ShaderOptions) -> Result<Ref<Shader>> {
        let framebuffer = &self.window_framebuffers.lock().unwrap()[0];
        let shader = Shader::new(
            &self.device,
            framebuffer,
            &self.shader_layout,
            source,
            options,
        )?;
        Ok(self.resources.add_shader(shader))
    }

    pub fn create_shader_from_file(
        &mut self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Result<Ref<Shader>> {
        let source = fs::read(path.as_ref())?;
        self.create_shader(&source, options)
    }

    pub fn render_stats(&self) -> RenderStats {
        self.render_stats
    }

    fn begin_draw(&mut self) -> Result<()> {
        self.render_stage = RenderStage::During;
        self.render_stats = Default::default();
        self.device.next_frame(&mut self.swapchain)?;
        self.resources.clean_unused(&mut self.image_uniform);
        self.resources.update_if_needed()?;
        self.image_uniform.update_if_needed();
        self.device.cmd_bind_uniform(
            self.device.command_buffer(),
            &self.shader_layout,
            &self.image_uniform,
        );

        #[cfg(feature = "ui")]
        self.ui.as_mut().unwrap().reset();

        Ok(())
    }

    fn end_draw(&mut self) -> Result<()> {
        self.render_stage = RenderStage::Before;
        self.device.submit()?;
        self.device.present(&self.swapchain)?;

        Ok(())
    }

    #[cfg(feature = "window")]
    pub fn with_window(
        c_options: ContextOptions,
        w_options: WindowOptions<'_>,
    ) -> Result<(Self, Window)> {
        use glfw::ClientApiHint;
        use glfw::WindowHint;
        use glfw::WindowMode;

        let WindowOptions {
            title,
            width,
            height,
        } = w_options;

        // create glfw window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

        let (mut window, event_receiver) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
            .unwrap();

        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_char_polling(true);

        // create context
        #[cfg(target_os = "windows")]
        let handle = WindowHandle {
            hwnd: window.get_win32_window(),
            width,
            height,
        };

        #[cfg(target_os = "linux")]
        let handle = WindowHandle {
            xlib_window: window.xlib_window(),
            xlib_display: window.xlib_display(),
            width,
            height,
        };

        #[cfg(target_os = "macos")]
        let handle = WindowHandle {
            ns_window: window.ns_window(),
            ns_view: window.ns_view(),
            width,
            height,
        };
        let mut context = Self::new(handle, c_options)?;

        // attach glfw to context
        context.attach_glfw(glfw, event_receiver);

        // create ui renderer
        #[cfg(feature = "ui")]
        context.attach_ui(width, height)?;

        Ok((context, Window::new(window)))
    }

    #[cfg(feature = "window")]
    pub(crate) fn attach_glfw(
        &mut self,
        glfw: glfw::Glfw,
        event_receiver: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    ) {
        self.glfw = Some(glfw);
        self.event_receiver = Some(event_receiver);
    }

    #[cfg(feature = "ui")]
    pub(crate) fn attach_ui(&mut self, width: u32, height: u32) -> Result<()> {
        let ui = Ui::new(
            &self.device,
            &self.shader_layout,
            &mut self.image_uniform,
            &mut self.resources,
            width,
            height,
        )?;
        self.ui = Some(ui);

        Ok(())
    }

    #[cfg(feature = "window")]
    pub fn poll_events(&mut self, window: &mut Window) -> Result<()> {
        use glfw::WindowEvent;
        use std::time::Duration;
        use std::time::Instant;

        // clear events
        window.clear();

        // poll events
        let mut polling = true;
        while polling {
            self.glfw.as_mut().unwrap().poll_events();
            for (_, event) in glfw::flush_messages(self.event_receiver.as_ref().unwrap()) {
                // update imgui
                #[cfg(feature = "ui")]
                self.ui.as_mut().unwrap().handle_event(&event);

                // update window events
                match event {
                    WindowEvent::Key(key, _, action, _) => window.handle_key(key, action),
                    WindowEvent::CursorPos(x, y) => window.handle_mouse(x, y),
                    WindowEvent::Scroll(x, y) => window.handle_scroll(x, y),
                    WindowEvent::Size(w, h) if w != 0 && h != 0 => window.record_resize(),
                    WindowEvent::MouseButton(button, action, _) => {
                        window.handle_mouse_button(button, action)
                    }
                    _ => (),
                }
            }

            // check resize timing
            if let Some(last) = window.last_resize() {
                if Instant::now().duration_since(last) >= Duration::from_millis(100) {
                    let (w, h) = window.raw_size();
                    self.resize(w as u32, h as u32)?;
                    window.handle_resize(w as u32, h as u32);
                    window.reset_resize();

                    info!("resized window to {}x{}", w, h);
                }
            }

            // pause if just resized
            polling = window.raw_size() == (0, 0) || window.last_resize().is_some();
        }

        // update window
        window.update_delta_time();

        Ok(())
    }

    #[cfg(feature = "hot-reload")]
    pub fn create_shader_from_file_watch(
        &mut self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Result<Ref<Shader>> {
        use notify::RecommendedWatcher;
        use notify::RecursiveMode;
        use notify::Watcher;
        use std::thread;
        use std::time::Duration;

        let path_buf = path.as_ref().to_path_buf();
        let shader = self.create_shader_from_file(&path_buf, options)?;

        // setup watcher
        let framebuffers = self.window_framebuffers.clone();
        let shader_layout = self.shader_layout.clone();
        let device = self.device.clone();
        let shader_ref = shader.clone();
        let (sender, receiver) = mpsc::channel();
        self.stop_senders.push(sender.clone());

        thread::spawn(move || {
            let mut watcher: RecommendedWatcher =
                Watcher::new(sender, Duration::from_millis(500)).unwrap();
            watcher
                .watch(&path_buf, RecursiveMode::NonRecursive)
                .unwrap();

            while let Ok(event) = receiver.recv() {
                match event {
                    DebouncedEvent::Rescan => break,
                    DebouncedEvent::Write(_) => {
                        // recreate shader
                        let framebuffer = &framebuffers.lock().unwrap()[0];

                        let source = fs::read(&path_buf).unwrap();
                        let new_shader =
                            Shader::new(&device, framebuffer, &shader_layout, &source, options)
                                .unwrap();
                        shader_ref.with(|s| *s = new_shader);
                        info!("shader {:?} was reloaded", path_buf);
                    }
                    _ => (),
                }
            }
        });

        Ok(shader)
    }

    #[cfg(feature = "image")]
    pub fn create_texture_from_file(&mut self, path: impl AsRef<Path>) -> Result<Ref<Texture>> {
        use image_file::GenericImageView;

        let img = image_file::open(path)?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();

        let texture = Texture::new(
            &self.device,
            &mut self.image_uniform,
            TextureOptions {
                format: ImageFormat::Srgba,
                data: &data,
                width,
                height,
            },
        )?;
        Ok(self.resources.add_texture(texture))
    }

    #[cfg(feature = "image")]
    pub fn set_skybox_from_file(&mut self, paths: [impl AsRef<Path>; 6]) -> Result<()> {
        use image_file::GenericImageView;

        let mut size = 0;
        let mut data = vec![];
        for path in &paths {
            let img = image_file::open(path)?;
            size = img.dimensions().0;
            data.push(img.to_rgba().into_raw());
        }

        let mut cubemap = Cubemap::new(
            &self.device,
            CubemapOptions {
                format: ImageFormat::Srgba,
                top: &data[0],
                bottom: &data[1],
                front: &data[2],
                back: &data[3],
                left: &data[4],
                right: &data[5],
                size,
            },
        )?;

        self.image_uniform.set_skybox(cubemap.add_view()?);
        self.skybox = cubemap;

        Ok(())
    }

    #[cfg(feature = "ui")]
    pub fn draw_ui(&mut self, draw_fn: impl FnMut(&imgui::Ui<'_>)) -> Result<()> {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw()?;
        }

        self.ui
            .as_mut()
            .unwrap()
            .draw(&self.shader_layout, draw_fn)?;

        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        #[cfg(feature = "hot-reload")]
        for stop in &self.stop_senders {
            stop.send(DebouncedEvent::Rescan).unwrap();
        }
        self.device.wait_for_idle().unwrap();
    }
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            quality: Quality::Medium,
            vsync: true,
            camera: CameraType::Perspective,
        }
    }
}

#[cfg(feature = "window")]
impl Default for WindowOptions<'_> {
    fn default() -> Self {
        Self {
            title: "Default title",
            width: 500,
            height: 500,
        }
    }
}
