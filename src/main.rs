mod character;
mod obstacle;

use character::{Character, CharacterType};
use obstacle::Obstacle;
use raylib::prelude::*;

struct GameState {
    characters: Vec<Character>,
    obstacles: Vec<Obstacle>,
    camera: Camera2D,
    is_debug: bool,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 620)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let camera = camera::Camera2D {
        rotation: 0.0,
        offset: Vector2::zero(),
        zoom: 1.0,
        target: Vector2::zero(),
    };
    let mut game_state = GameState {
        characters: vec![],
        obstacles: vec![],
        camera: camera,
        is_debug: true,
    };

    let player = Character::new(
        CharacterType::Player,
        Vector2::new(
            (rl.get_render_width() / 2) as f32,
            (rl.get_render_height() / 2) as f32,
        ),
        200.0,
    );
    let enemy = Character::new(CharacterType::Enemy, Vector2::new(100.0, 100.0), 100.0);

    game_state.characters.push(player);
    game_state.characters.push(enemy);

    game_state.obstacles.push(Obstacle {
        position: Vector2::new(200.0, 200.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(280.0, 200.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(400.0, 100.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(464.0, 200.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(400.0, 300.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(464.0, 400.0),
        radius: 32.0,
    });

    while !rl.window_should_close() {
        update(&rl, &mut game_state);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &game_state);
    }
}

fn update(rl: &RaylibHandle, game_state: &mut GameState) {
    let delta = rl.get_frame_time();
    let mouse_pos = rl.get_mouse_position();
    let mouse_pos = rl.get_screen_to_world2D(mouse_pos, game_state.camera);
    let player_pos = game_state.characters.first().unwrap().position;

    game_state.camera.target = Vector2 {
        x: player_pos.x - rl.get_screen_width() as f32 / 2.0,
        y: player_pos.y - rl.get_screen_height() as f32 / 2.0,
    };

    if rl.is_key_pressed(KeyboardKey::KEY_O) {
        game_state.is_debug = !game_state.is_debug;
    }

    for character in game_state.characters.iter_mut() {
        match character.character_type {
            CharacterType::Player => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                    character.position = mouse_pos;
                }
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    character.set_travel_target_position(mouse_pos, &game_state.obstacles);
                }
            }
            CharacterType::Enemy => {
                let last_route_point = character.path.last();
                match last_route_point {
                    Some(&point) => {
                        if (point - player_pos).length() > 50.0 {
                            character.set_travel_target_position(player_pos, &game_state.obstacles);
                        }
                    }
                    None => {
                        character.set_travel_target_position(player_pos, &game_state.obstacles);
                    }
                }
            }
        }
        character.move_character(delta);
    }

    for obstacle in game_state.obstacles.iter() {
        for character in game_state.characters.iter_mut() {
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

fn draw(d: &mut RaylibDrawHandle, game_state: &GameState) {
    d.clear_background(Color::WHITE);
    let mut ordered_characters: Vec<&Character> = game_state.characters.iter().collect();
    ordered_characters.sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    d.draw_mode2D(game_state.camera, |mut mode_2d| {
        for obstacle in game_state.obstacles.iter() {
            mode_2d.draw_circle(
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
            mode_2d.draw_circle(
                character.position.x as i32,
                character.position.y as i32,
                character.radius,
                color,
            );
        }

        if game_state.is_debug {
            for obstacle in game_state.obstacles.iter() {
                mode_2d.draw_circle(
                    obstacle.position.x as i32,
                    obstacle.position.y as i32,
                    obstacle.radius + ordered_characters.first().unwrap().radius,
                    Color::BLACK.alpha(0.5),
                );
            }
            for character in game_state.characters.iter() {
                let color = match character.character_type {
                    CharacterType::Player => Color::LIME,
                    CharacterType::Enemy => Color::HOTPINK,
                };
                for (i, waypoint) in character.path.iter().enumerate() {
                    if i == 0 {
                        mode_2d.draw_line(
                            character.position.x as i32,
                            character.position.y as i32,
                            waypoint.x as i32,
                            waypoint.y as i32,
                            color,
                        );
                    } else if i < character.path.len() {
                        mode_2d.draw_line(
                            character.path.get(i - 1).unwrap().x as i32,
                            character.path.get(i - 1).unwrap().y as i32,
                            waypoint.x as i32,
                            waypoint.y as i32,
                            color,
                        );
                    }
                    mode_2d.draw_circle(waypoint.x as i32, waypoint.y as i32, 4.0, color);
                    mode_2d.draw_text(
                        format!("{}, {}", waypoint.x, waypoint.y).as_str(),
                        waypoint.x as i32 + 12,
                        waypoint.y as i32,
                        16,
                        color,
                    );
                }
            }
        }
    });

    d.draw_fps(4, 4);
}
