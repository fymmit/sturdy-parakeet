mod character;

use character::{Character, CharacterType};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let mut characters: Vec<Character> = vec![];

    let player = Character::new(
        CharacterType::Player,
        Vector2::new(
            (rl.get_render_width() / 2) as f32,
            (rl.get_render_height() / 2) as f32,
        ),
        200.0,
    );
    let enemy = Character::new(CharacterType::Enemy, Vector2::new(100.0, 100.0), 100.0);

    characters.push(player);
    characters.push(enemy);

    while !rl.window_should_close() {
        update(&rl, &mut characters);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &characters);
    }
}

fn update(rl: &RaylibHandle, characters: &mut [Character]) {
    let delta = rl.get_frame_time();
    let mouse_pos = rl.get_mouse_position();
    let player_pos = characters.first().unwrap().position;

    for character in characters.iter_mut() {
        match character.character_type {
            CharacterType::Player => {
                if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    character.set_travel_target_position(mouse_pos);
                }
            }
            CharacterType::Enemy => {
                character.set_travel_target_position(player_pos);
            }
        }
        character.move_character(delta);
    }
}

fn draw(d: &mut RaylibDrawHandle, characters: &[Character]) {
    d.clear_background(Color::WHITE);
    let mut ordered_characters: Vec<&Character> = characters.into_iter().collect();
    ordered_characters.sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    for character in ordered_characters.iter() {
        let color = match character.character_type {
            CharacterType::Player => Color::GREEN,
            CharacterType::Enemy => Color::RED,
        };
        d.draw_circle(
            character.position.x as i32,
            character.position.y as i32,
            16.0,
            color,
        );
    }

    d.draw_fps(4, 4);
}
