use tegne_math::Quaternion;
use tegne_math::Transform;
use tegne_math::Vector3;

use super::Events;
use super::Key;

#[derive(Default)]
pub struct Controller {
    camera_angle: f32,
    mouse_grab: bool,
    lockon_point: Vector3,
    lockon: bool,
}

impl Controller {
    pub fn update(&mut self, camera: &mut Transform, events: &Events) {
        let move_speed = 10.0;
        let rotate_speed = 50.0;
        let mut speed_mod = 1.0;

        if events.is_key_typed(Key::Escape) {
            self.mouse_grab = !self.mouse_grab;
            events.set_mouse_grab(self.mouse_grab);
            events.set_mouse_visible(!self.mouse_grab);
        }

        if events.is_key_typed(Key::LAlt) {
            self.lockon = !self.lockon;
        }

        if events.is_key_pressed(Key::LShift) {
            speed_mod = 5.0;
        }

        // camera movement
        let final_move_speed = move_speed * speed_mod * events.delta_time();

        if events.is_key_pressed(Key::W) {
            camera.move_forward(final_move_speed);
        }

        if events.is_key_pressed(Key::S) {
            camera.move_backward(final_move_speed);
        }

        if events.is_key_pressed(Key::A) {
            camera.move_left(final_move_speed);
        }

        if events.is_key_pressed(Key::D) {
            camera.move_right(final_move_speed);
        }

        if events.is_key_pressed(Key::Space) {
            camera.move_up(final_move_speed);
        }

        if events.is_key_pressed(Key::LControl) {
            camera.move_down(final_move_speed);
        }

        // look direction
        if self.mouse_grab {
            let (x, y) = events.mouse_delta();

            let mouse_x = x * rotate_speed * events.delta_time();

            let change_y = y * rotate_speed * events.delta_time();
            let upper_bound = change_y + self.camera_angle <= 90.0;
            let lower_bound = change_y + self.camera_angle >= -90.0;
            let mouse_y = if upper_bound && lower_bound {
                self.camera_angle += change_y;
                change_y
            } else {
                0.0
            };

            let pitch = Quaternion::euler_rotation(0.0, mouse_x, 0.0);
            let roll = Quaternion::euler_rotation(mouse_y, 0.0, 0.0);

            camera.rotation = pitch * camera.rotation * roll;
        }

        if self.lockon {
            camera.look_at(self.lockon_point, Vector3::up());
        }
    }
}