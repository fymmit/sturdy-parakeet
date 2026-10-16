use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let mut position = Vector2::new(0.0, 0.0);
    let movement_speed = 200.0;

    while !rl.window_should_close() {
        let delta = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();
        position += get_input(&rl) * movement_speed * delta;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 16.0, Color::RED);
        d.draw_circle(position.x as i32, position.y as i32, 16.0, Color::GREEN);

        d.draw_fps(4, 4);
    }
}

fn get_input(d: &RaylibHandle) -> Vector2 {
    let mut x = 0.0;
    let mut y = 0.0;
    if d.is_key_down(KeyboardKey::KEY_A) {
        x -= 1.0;
    }
    if d.is_key_down(KeyboardKey::KEY_D) {
        x += 1.0;
    }
    if d.is_key_down(KeyboardKey::KEY_W) {
        y -= 1.0;
    }
    if d.is_key_down(KeyboardKey::KEY_S) {
        y += 1.0;
    }

    Vector2::new(x, y).normalize()
}
