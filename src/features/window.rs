// Oliver Berzs
// https://github.com/oberzs/duku

//! Optional feature `window` module for simple window creation.

#![cfg(feature = "window")]

use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;
use window_dep::dpi::PhysicalPosition;
use window_dep::dpi::PhysicalSize;
use window_dep::event::DeviceEvent;
use window_dep::event::ElementState;
use window_dep::event::Event as WinitEvent;
use window_dep::event::MouseScrollDelta;
use window_dep::event::WindowEvent;
use window_dep::event_loop::ControlFlow;
use window_dep::event_loop::EventLoop;
use window_dep::window::Window as WinitWindow;
use window_dep::window::WindowBuilder as WinitWindowBuilder;

pub use window_dep::event::MouseButton;
pub use window_dep::event::VirtualKeyCode as Key;
pub use window_dep::window::CursorIcon as Cursor;

use crate::duku::Duku;
use crate::duku::DukuBuilder;
use crate::error::Result;
use crate::math::Vec2;
use crate::math::Vec3;
use crate::renderer::Camera;
use crate::surface::WindowHandle;

/// OS window wrapper around `winit`.
pub struct Window {
    window: WinitWindow,
    event_loop: EventLoop<()>,
}

/// OS window event handler.
pub struct Events {
    window: WinitWindow,
    events: Vec<Event>,

    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,
    keys_typed: HashSet<Key>,
    buttons_pressed: HashSet<MouseButton>,
    buttons_released: HashSet<MouseButton>,
    buttons_clicked: HashSet<MouseButton>,
    typed_char: Option<char>,

    mouse_position: Vec2,
    mouse_delta: Vec2,
    mouse_grab: bool,
    scroll_delta: Vec2,
}

/// OS window event.
#[derive(Debug, Copy, Clone)]
pub enum Event {
    /// window resize event
    Resize(Vec2),
}

/// Simple orbit camera controller.
#[derive(Debug, Copy, Clone)]
pub struct Orbit {
    /// center point to rotate around
    pivot: Vec3,
    /// camera move speed
    move_speed: f32,
}

/// OS window builder.
#[derive(Debug, Clone)]
pub struct WindowBuilder {
    duku: DukuBuilder,
    title: String,
    resizable: bool,
    width: u32,
    height: u32,
}

impl Duku {
    /// Create Duku with a basic window
    pub fn windowed(width: u32, height: u32) -> Result<(Duku, Window)> {
        Self::builder().build_window(width, height).build()
    }
}

impl DukuBuilder {
    /// Create OS window builder
    pub fn build_window(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            duku: self,
            title: "".to_string(),
            resizable: false,
            width,
            height,
        }
    }
}

impl Window {
    pub(crate) fn new(title: &str, width: u32, height: u32, resizable: bool) -> Self {
        let event_loop = EventLoop::new();
        let window = WinitWindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_title(title)
            .with_resizable(resizable)
            .build(&event_loop)
            .expect("bad window");

        Self { window, event_loop }
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn handle(&self) -> WindowHandle {
        use window_dep::platform::windows::WindowExtWindows;

        WindowHandle {
            hwnd: self.window.hwnd(),
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn handle(&self) -> WindowHandle {
        use window_dep::platform::unix::WindowExtUnix;

        WindowHandle {
            xlib_window: self.window.xlib_window().expect("Wayland not supported"),
            xlib_display: self.window.xlib_display().expect("Wayland not supported"),
        }
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn handle(&self) -> WindowHandle {
        unimplemented!()
    }

    /// Start window's main loop for polling events
    pub fn while_open<F>(self, mut main_fn: F)
    where
        F: FnMut(&mut Events) + 'static,
    {
        let Self { window, event_loop } = self;
        let mut events = Events {
            events: vec![],
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            keys_typed: HashSet::new(),
            buttons_pressed: HashSet::new(),
            buttons_released: HashSet::new(),
            buttons_clicked: HashSet::new(),
            mouse_position: Vec2::default(),
            mouse_delta: Vec2::default(),
            mouse_grab: false,
            scroll_delta: Vec2::default(),
            typed_char: None,
            window,
        };

        let mut last_resize = None;

        event_loop.run(move |event, _, control_flow| match event {
            WinitEvent::WindowEvent { event, window_id } if window_id == events.window.id() => {
                match event {
                    // close event
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    // resize event
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            last_resize = Some(Instant::now());
                        }
                    }

                    // mouse position event
                    WindowEvent::CursorMoved { position, .. } => {
                        events.mouse_position = Vec2::new(position.x as f32, position.y as f32);
                    }

                    // keyboard key event
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => {
                                    events.keys_typed.insert(key);
                                    events.keys_pressed.insert(key);
                                    events.keys_released.remove(&key);
                                }
                                ElementState::Released => {
                                    events.keys_released.insert(key);
                                    events.keys_pressed.remove(&key);
                                    events.keys_typed.remove(&key);
                                }
                            }
                        }
                    }

                    // mouse button event
                    WindowEvent::MouseInput { state, button, .. } => match state {
                        ElementState::Pressed => {
                            events.buttons_clicked.insert(button);
                            events.buttons_pressed.insert(button);
                            events.buttons_released.remove(&button);
                        }
                        ElementState::Released => {
                            events.buttons_released.insert(button);
                            events.buttons_pressed.remove(&button);
                            events.buttons_clicked.remove(&button);
                        }
                    },

                    // text input event
                    WindowEvent::ReceivedCharacter(c) => {
                        if !c.is_ascii_control() {
                            events.typed_char = Some(c);
                        }
                    }

                    // mouse scroll event
                    WindowEvent::MouseWheel { delta, .. } => {
                        if let MouseScrollDelta::LineDelta(x, y) = delta {
                            events.scroll_delta = Vec2::new(x as f32, y as f32);
                        }
                    }

                    _ => (),
                }
            }

            // mouse delta event
            WinitEvent::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    let (x, y) = delta;
                    events.mouse_delta = Vec2::new(x as f32, y as f32);
                }
            }

            // draw event
            WinitEvent::MainEventsCleared => {
                // check resize timing
                if let Some(last) = last_resize {
                    if Instant::now().duration_since(last) >= Duration::from_millis(100) {
                        let size = events.size();
                        events.events.push(Event::Resize(size));
                        last_resize = None;

                        info!("resized window to {}x{}", size.x as u32, size.y as u32);
                    }
                }

                let size = events.size();
                if size.x as i32 != 0 && size.y as i32 != 0 && last_resize == None {
                    main_fn(&mut events);
                }

                events.events.clear();
                events.keys_typed.clear();
                events.typed_char = None;
                events.mouse_delta = Vec2::new(0.0, 0.0);
                events.scroll_delta = Vec2::new(0.0, 0.0);
            }
            _ => (),
        });
    }
}

impl Events {
    /// Check if keyboard key is pressed
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check if keyboard key is released
    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    /// Check if keyboard key has been typed
    pub fn is_key_typed(&self, key: Key) -> bool {
        self.keys_typed.contains(&key)
    }

    /// Check if mouse button is pressed
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button)
    }

    /// Check if mouse button is released
    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.buttons_released.contains(&button)
    }

    /// Check if mouse button has been clicked
    pub fn is_button_clicked(&self, button: MouseButton) -> bool {
        self.buttons_clicked.contains(&button)
    }

    /// Get mouse position
    pub const fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// Set mouse position
    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.window
            .set_cursor_position(PhysicalPosition::new(position.x as i32, position.y as i32))
            .expect("cannot set cursor position");
    }

    /// Get mouse position's change since last frame
    pub const fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    /// Get scroll wheel's change since last frame
    pub const fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    /// Get if mouse is contained in window
    pub const fn mouse_grab(&self) -> bool {
        self.mouse_grab
    }

    /// Set if mouse is contained in window
    pub fn set_mouse_grab(&mut self, grab: bool) {
        self.window
            .set_cursor_grab(grab)
            .expect("cannot set cursor grab");
        self.mouse_grab = grab;
    }

    /// Set if cursor is hidden
    pub fn hide_cursor(&mut self, hide: bool) {
        self.window.set_cursor_visible(!hide);
    }

    /// Set cursor icon
    pub fn set_cursor(&mut self, cursor: Cursor) {
        self.window.set_cursor_icon(cursor);
    }

    /// Get window size
    pub fn size(&self) -> Vec2 {
        let size = self.window.inner_size();
        Vec2::new(size.width as f32, size.height as f32)
    }

    /// Iterate over window events
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }

    /// Set window title
    pub fn set_title(&mut self, title: impl AsRef<str>) {
        self.window.set_title(title.as_ref());
    }

    /// Get typed character if there is one
    pub const fn typed_char(&self) -> Option<char> {
        self.typed_char
    }
}

impl Orbit {
    /// Create a orbit controller
    pub fn new(pivot: impl Into<Vec3>) -> Self {
        Self {
            pivot: pivot.into(),
            move_speed: 2.5,
        }
    }

    /// Update camera
    pub fn update(&mut self, camera: &mut Camera, events: &mut Events, delta_time: f32) {
        // control orbiting around pivot
        let angle = 5.0f32.powf(self.move_speed) * delta_time;

        // mouse rotation
        if events.is_button_pressed(MouseButton::Middle) {
            // toggle mouse grab if needed
            if !events.mouse_grab() {
                events.set_mouse_grab(true);
                events.hide_cursor(true);
            }

            let delta = events.mouse_delta();
            let speed = 50.0 * delta_time;

            camera.move_around_point(self.pivot, speed * delta.x, Vec3::up());
            camera.move_around_point(self.pivot, speed * delta.y, camera.rotation.local_right());
        } else {
            // toggle mouse grab if needed
            if events.mouse_grab() {
                events.set_mouse_grab(false);
                events.hide_cursor(false);
            }
        }

        // horizontal rotation
        if events.is_key_pressed(Key::D) {
            camera.move_around_point(self.pivot, -angle, Vec3::up());
        }
        if events.is_key_pressed(Key::A) {
            camera.move_around_point(self.pivot, angle, Vec3::up());
        }

        // vertical rotation
        if events.is_key_pressed(Key::W) {
            camera.move_around_point(self.pivot, angle, camera.rotation.local_right());
        }
        if events.is_key_pressed(Key::S) {
            camera.move_around_point(self.pivot, -angle, camera.rotation.local_right());
        }

        // zoom
        let scroll = events.scroll_delta();
        camera.move_forward(scroll.y * (self.pivot - camera.position).length() * 0.05);

        // look at pivot point
        camera.look_at(self.pivot);
    }
}

impl WindowBuilder {
    /// Make window resizable
    pub const fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    /// Use window title
    pub fn title(mut self, title: impl AsRef<str>) -> Self {
        self.title = title.as_ref().to_string();
        self
    }

    /// Build duku context and window
    pub fn build(self) -> Result<(Duku, Window)> {
        let window = Window::new(&self.title, self.width, self.height, self.resizable);
        let duku = self.duku.attach_window(window.handle()).build()?;

        Ok((duku, window))
    }
}
