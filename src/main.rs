use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let mut player = Player {
        position: Vector2::new(
            (rl.get_render_width() / 2) as f32,
            (rl.get_render_height() / 2) as f32,
        ),
        movement_speed: 200.0,
        target_position: Vector2::zero(),
        is_traveling: false,
    };

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            player.set_target_position(mouse_pos);
        }

        player.move_player(delta);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_circle(
            player.position.x as i32,
            player.position.y as i32,
            16.0,
            Color::GREEN,
        );

        d.draw_fps(4, 4);
    }
}

// fn get_movement_direction(d: &RaylibHandle) -> Vector2 {
//     let mut x = 0.0;
//     let mut y = 0.0;
//     if d.is_key_down(KeyboardKey::KEY_A) {
//         x -= 1.0;
//     }
//     if d.is_key_down(KeyboardKey::KEY_D) {
//         x += 1.0;
//     }
//     if d.is_key_down(KeyboardKey::KEY_W) {
//         y -= 1.0;
//     }
//     if d.is_key_down(KeyboardKey::KEY_S) {
//         y += 1.0;
//     }
//
//     Vector2::new(x, y).normalize()
// }

struct Player {
    position: Vector2,
    movement_speed: f32,
    target_position: Vector2,
    is_traveling: bool,
}

impl Player {
    fn set_target_position(&mut self, target: Vector2) {
        self.target_position = target;
        self.is_traveling = true;
    }
    fn move_player(&mut self, delta: f32) {
        if self.is_traveling {
            let difference = self.target_position - self.position;
            let direction = difference.normalize();
            self.position += direction * self.movement_speed * delta;
            if difference.length() <= 5.0 {
                self.is_traveling = false;
            }
        }
    }
}
