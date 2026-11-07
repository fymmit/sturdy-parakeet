use raylib::prelude::*;

use crate::WINDOW_WIDTH;
use crate::character::{Character, CharacterType};
use crate::world::chunks::ChunkManager;
use crate::world::tile::Tile;

pub struct GameState {
    pub characters: Vec<Character>,
    pub camera: Camera2D,
    pub chunk_manager: ChunkManager,
    pub visible_tiles: Vec<Tile>,
    pub is_debug: bool,
}

impl GameState {
    pub fn update(&mut self, rl: &RaylibHandle) {
        let delta = rl.get_frame_time();

        let mouse_pos = rl.get_mouse_position();
        let mouse_pos = rl.get_screen_to_world2D(mouse_pos, self.camera);
        let player_pos = self.characters.first().unwrap().position;

        self.camera.target = Vector2 {
            x: player_pos.x,
            y: player_pos.y,
        };

        if rl.is_key_pressed(KeyboardKey::KEY_O) {
            self.is_debug = !self.is_debug;
        }

        let left_mouse_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        if !self.chunk_manager.is_chunk_generated(mouse_pos) {
            self.chunk_manager.generate_chunk(mouse_pos);
        }

        let mouse_wheel_move = rl.get_mouse_wheel_move();
        if mouse_wheel_move < 0.0 {
            self.camera.zoom -= 0.1;
        } else if mouse_wheel_move > 0.0 {
            self.camera.zoom += 0.1;
        }

        for character in self.characters.iter_mut() {
            match character.character_type {
                CharacterType::Player => {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                        character.position = mouse_pos;
                    }
                    if left_mouse_clicked {
                        character.set_target_position(
                            mouse_pos,
                            &self.chunk_manager,
                            &self.visible_tiles,
                        );
                    }
                }
                CharacterType::Enemy => {
                    let last_route_point = character.path.last();
                    match last_route_point {
                        Some(&point) => {
                            if (point - player_pos).length() > 50.0 {
                                character.set_target_position(
                                    player_pos,
                                    &self.chunk_manager,
                                    &self.visible_tiles,
                                );
                            }
                        }
                        None => {
                            if (character.position - player_pos).length() > 50.0 {
                                character.set_target_position(
                                    player_pos,
                                    &self.chunk_manager,
                                    &self.visible_tiles,
                                );
                            }
                        }
                    }
                }
            }
            character.move_character(delta);
        }

        self.visible_tiles = self
            .chunk_manager
            .get_tiles_from_chunks_around_position(player_pos)
            .into_iter()
            .filter(|tile| {
                self.camera.target.distance(Vector2::new(
                    tile.global_position_x as f32,
                    tile.global_position_y as f32,
                )) < WINDOW_WIDTH as f32
            })
            .collect();
    }
}
