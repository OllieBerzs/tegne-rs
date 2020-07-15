// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Toon shader example

use tegne::camera::Controller;
use tegne::color::colors;
use tegne::math::Transform;
use tegne::math::Vector3;
use tegne::window::Window;
use tegne::window::WindowOptions;
use tegne::Context;
use tegne::ContextOptions;

fn main() {
    let (width, height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Toon",
        width,
        height,
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            anisotropy: 16.0,
            msaa: 4,
            ..Default::default()
        },
    );

    let texture = context
        .create_texture_from_file("examples/toon/textures/texture_09.png")
        .unwrap();

    let shader = context
        .create_shader_from_file_watch("examples/toon/shaders/toon.shader", Default::default())
        .unwrap();

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_backward(5.0);
        cam_t.move_up(2.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut controller = Controller::default();

    let floor_transform = Transform {
        scale: Vector3::new(10.0, 0.2, 10.0),
        ..Default::default()
    };

    window.main_loop(|events, _| {
        controller.update(&mut context.main_camera, events);

        context.draw_on_window(|target| {
            target.set_clear(colors::SKY_BLUE);

            // floor
            target.draw_cube(floor_transform);

            // toon cube and sphere
            target.set_shader(&shader);
            target.draw_cube([-3.0, 0.6, 0.0]);
            target.draw_sphere([-1.0, 1.0, 0.0]);
            target.set_shader_phong();

            // textured cube and sphere
            target.set_albedo(&texture);
            target.draw_cube([1.0, 1.0, 0.0]);
            target.draw_sphere([3.0, 1.0, 0.0]);
        });
    });
}
