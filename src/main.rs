mod character;
mod chunks;
mod obstacle;
mod tile;

use raylib::prelude::*;

use character::{Character, CharacterType};
use chunks::{CHUNK_SIZE, CHUNK_TILE_SIZE, ChunkManager};
use obstacle::Obstacle;
use tile::TileType;

struct GameState {
    characters: Vec<Character>,
    obstacles: Vec<Obstacle>,
    camera: Camera2D,
    chunk_manager: ChunkManager,
    is_debug: bool,
}

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let camera = camera::Camera2D {
        rotation: 0.0,
        offset: Vector2::new((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32),
        zoom: 1.0,
        target: Vector2::zero(),
    };
    let mut game_state = GameState {
        characters: vec![],
        obstacles: vec![],
        camera: camera,
        chunk_manager: ChunkManager::new("memes"),
        is_debug: true,
    };

    generate_initial_chunks(&mut game_state);

    let player = Character::new(
        CharacterType::Player,
        Vector2::new((CHUNK_TILE_SIZE / 2) as f32, (CHUNK_TILE_SIZE / 2) as f32),
        200.0,
    );
    let enemy = Character::new(CharacterType::Enemy, Vector2::new(100.0, 100.0), 100.0);

    game_state.characters.push(player);
    game_state.characters.push(enemy);

    game_state.obstacles.push(Obstacle {
        position: Vector2::new(0.0, 200.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(80.0, 200.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(200.0, 100.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(264.0, 200.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(200.0, 300.0),
        radius: 32.0,
    });
    game_state.obstacles.push(Obstacle {
        position: Vector2::new(264.0, 400.0),
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
        x: player_pos.x,
        y: player_pos.y,
    };

    if rl.is_key_pressed(KeyboardKey::KEY_O) {
        game_state.is_debug = !game_state.is_debug;
    }

    let left_mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    if left_mouse_clicked {
        if !game_state.chunk_manager.is_chunk_generated(mouse_pos) {
            game_state.chunk_manager.generate_chunk(mouse_pos);
        }
    }

    let selected_tile = game_state.chunk_manager.get_tile(mouse_pos);

    let mouse_wheel_move = rl.get_mouse_wheel_move();
    if mouse_wheel_move < 0.0 {
        game_state.camera.zoom -= 0.1;
    } else if mouse_wheel_move > 0.0 {
        game_state.camera.zoom += 0.1;
    }

    for character in game_state.characters.iter_mut() {
        match character.character_type {
            CharacterType::Player => {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                    character.position = mouse_pos;
                }
                if left_mouse_clicked {
                    if let Some(tile) = selected_tile {
                        println!("{:?}", tile);
                        let tile_center = tile.get_center();
                        character.set_travel_target_position(tile_center, &game_state.obstacles);
                    } else {
                        character.set_travel_target_position(mouse_pos, &game_state.obstacles);
                    }
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
        for chunk in game_state.chunk_manager.chunks.iter() {
            for tile in chunk.tiles.iter() {
                let x = tile.global_x;
                let y = tile.global_y;
                let color = match tile.tile_type {
                    TileType::Empty => Color::WHITE,
                    TileType::Obstacle => Color::BLACK,
                };
                mode_2d.draw_rectangle(x, y, CHUNK_TILE_SIZE, CHUNK_TILE_SIZE, color);
                mode_2d.draw_rectangle_lines(x, y, CHUNK_TILE_SIZE, CHUNK_TILE_SIZE, Color::GRAY);
            }
        }

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
            for chunk in game_state.chunk_manager.chunks.iter() {
                for tile in chunk.tiles.iter() {
                    mode_2d.draw_text(
                        format!("{},{}", tile.global_x, tile.global_y).as_str(),
                        tile.global_x,
                        tile.global_y,
                        16,
                        Color::GRAY,
                    );
                }
            }
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

fn generate_initial_chunks(game_state: &mut GameState) {
    game_state
        .chunk_manager
        .generate_chunk(Vector2::new(0.0, 0.0));
    game_state
        .chunk_manager
        .generate_chunk(Vector2::new(-1.0, 0.0));
    game_state
        .chunk_manager
        .generate_chunk(Vector2::new(0.0, -1.0));
    game_state
        .chunk_manager
        .generate_chunk(Vector2::new(-1.0, -1.0));
}
