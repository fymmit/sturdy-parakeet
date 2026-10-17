mod character;
mod obstacle;

use character::{Character, CharacterType};
use obstacle::Obstacle;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let mut characters: Vec<Character> = vec![];
    let mut obstacles: Vec<Obstacle> = vec![];

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

    obstacles.push(Obstacle {
        position: Vector2::new(200.0, 200.0),
        radius: 32.0,
    });
    obstacles.push(Obstacle {
        position: Vector2::new(280.0, 200.0),
        radius: 32.0,
    });
    obstacles.push(Obstacle {
        position: Vector2::new(400.0, 200.0),
        radius: 32.0,
    });

    while !rl.window_should_close() {
        update(&rl, &mut characters, &obstacles);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &characters, &obstacles);
    }
}

fn update(rl: &RaylibHandle, characters: &mut [Character], obstacles: &[Obstacle]) {
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

    for obstacle in obstacles.iter() {
        for character in characters.iter_mut() {
            while raylib::check_collision_circles(
                obstacle.position,
                obstacle.radius,
                character.position,
                16.0,
            ) {
                let push_direction = (character.position - obstacle.position).normalize();
                character.position += push_direction;
            }
        }
    }
}

fn draw(d: &mut RaylibDrawHandle, characters: &[Character], obstacles: &[Obstacle]) {
    d.clear_background(Color::WHITE);
    let mut ordered_characters: Vec<&Character> = characters.into_iter().collect();
    ordered_characters.sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    for obstacle in obstacles.iter() {
        d.draw_circle(
            obstacle.position.x as i32,
            obstacle.position.y as i32,
            obstacle.radius,
            Color::BLACK,
        );
    }

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
