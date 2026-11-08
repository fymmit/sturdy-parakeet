mod character;
mod game;
mod pathfinding;
mod rendering;
mod world;

use raylib::prelude::*;

use crate::character::{Character, CharacterType};
use crate::game::GameState;
use crate::world::chunks::{CHUNK_SIZE, ChunkManager};
use crate::world::tile::TILE_SIZE;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let camera = camera::Camera2D {
        rotation: -45.0,
        offset: Vector2::new((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32),
        zoom: 1.0,
        target: Vector2::zero(),
    };

    let mut game_state = GameState {
        characters: vec![],
        camera,
        chunk_manager: ChunkManager::new(),
        visible_tiles: vec![],
        is_debug: true,
    };

    game_state
        .chunk_manager
        .generate_chunk(Vector2::new(0.0, 0.0));

    let player = Character::new(
        CharacterType::Player,
        Vector2::new(
            (CHUNK_SIZE / 2 + TILE_SIZE / 2) as f32,
            (CHUNK_SIZE / 2 + TILE_SIZE / 2) as f32,
        ),
        200.0,
    );
    game_state.characters.push(player);

    let enemy = Character::new(CharacterType::Enemy, Vector2::new(100.0, 100.0), 100.0);
    game_state.characters.push(enemy);

    while !rl.window_should_close() {
        game_state.update(&rl);

        let mut d = rl.begin_drawing(&thread);

        rendering::draw(&mut d, &game_state);
    }
}
