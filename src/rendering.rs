use raylib::ffi::Color;
use raylib::prelude::*;

use crate::character::{Character, CharacterType};
use crate::game::GameState;
use crate::world::tile::{TILE_SIZE, TileType};

pub fn draw(d: &mut RaylibDrawHandle, game_state: &GameState) {
    d.clear_background(Color::WHITE);

    let mut ordered_characters: Vec<&Character> = game_state.characters.iter().collect();
    ordered_characters.sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    d.draw_mode2D(game_state.camera, |mut mode_2d| {
        for tile in game_state.visible_tiles.iter() {
            let x = tile.global_position_x;
            let y = tile.global_position_y;
            let color = match tile.tile_type {
                TileType::Empty => Color::WHITE,
                TileType::Obstacle => Color::BLACK,
                TileType::Water => Color::POWDERBLUE,
            };
            mode_2d.draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
            mode_2d.draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, Color::GRAY);
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
            for tile in game_state.visible_tiles.iter() {
                mode_2d.draw_text(
                    format!("{},{}", tile.tile_coords.x, tile.tile_coords.y).as_str(),
                    tile.global_position_x,
                    tile.global_position_y,
                    16,
                    Color::GRAY,
                );
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
