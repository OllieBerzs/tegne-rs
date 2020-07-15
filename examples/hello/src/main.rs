// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// "Hello, World!" example to open a rainbow resizable window

use tegne::color::Color;
use tegne::window::Window;
use tegne::window::WindowOptions;
use tegne::Context;

fn main() {
    let (mut width, mut height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Hello",
        resizable: true,
        width,
        height,
    });
    let mut context = Context::from_window(&mut window, Default::default());

    let mut hue = 0;

    window.main_loop(|events, _| {
        if let Some((new_width, new_height)) = events.resized() {
            context.resize(new_width, new_height);
            width = new_width;
            height = new_height;
        }

        hue = (hue + 1) % 360;

        context.draw_on_window(|target| {
            target.set_clear(Color::hsv(hue, 255, 255));
        });
    });
}
