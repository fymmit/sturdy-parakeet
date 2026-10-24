use rand::prelude::*;
use rand::rngs::SmallRng;
use raylib::ffi::{Rectangle, Vector2};

use crate::tile::{Coords, Tile, TileType};

pub const CHUNK_TILE_COUNT: i32 = 16;
pub const CHUNK_TILE_SIZE: i32 = 64;
pub const CHUNK_SIZE: i32 = CHUNK_TILE_COUNT * CHUNK_TILE_SIZE;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
}

pub struct ChunkManager {
    pub tiles: Vec<Tile>,
    chunks: Vec<Chunk>,
    rng: SmallRng,
}

impl ChunkManager {
    pub fn new(seed_str: &str) -> Self {
        let bytes: Vec<u8> = seed_str.bytes().collect();
        let mut seed: u64 = 0;
        for b in bytes.iter() {
            seed += *b as u64;
        }
        let rng = SmallRng::seed_from_u64(seed);
        Self {
            tiles: vec![],
            chunks: vec![],
            rng,
        }
    }
    pub fn generate_chunk(&mut self, position: Vector2) {
        let chunk_x = (position.x / CHUNK_SIZE as f32).floor() as i32;
        let x_coord = chunk_x * CHUNK_SIZE;
        let chunk_y = (position.y / CHUNK_SIZE as f32).floor() as i32;
        let y_coord = chunk_y * CHUNK_SIZE;
        println!("Generating a chunk at: {}, {}", chunk_x, chunk_y);
        let mut tiles: Vec<Tile> = vec![];
        for x in 0..CHUNK_TILE_COUNT {
            for y in 0..CHUNK_TILE_COUNT {
                let tile_should_be_obstacle = self.rng.random_range(0..5) == 0;
                let tile_type = if tile_should_be_obstacle {
                    TileType::Obstacle
                } else {
                    TileType::Empty
                };
                let x_position = x_coord + x * CHUNK_TILE_SIZE;
                let y_position = y_coord + y * CHUNK_TILE_SIZE;
                let x = chunk_x * CHUNK_TILE_COUNT + x;
                let y = chunk_y * CHUNK_TILE_COUNT + y;
                tiles.push(Tile {
                    tile_coords: Coords { x, y },
                    x_position,
                    y_position,
                    tile_type,
                });
            }
        }

        self.tiles.append(&mut tiles);

        self.chunks.push(Chunk {
            x: x_coord,
            y: y_coord,
        });
    }
    pub fn is_chunk_generated(&self, position: Vector2) -> bool {
        self.chunks.iter().any(|chunk| {
            let rec = Rectangle::new(
                chunk.x as f32,
                chunk.y as f32,
                CHUNK_SIZE as f32,
                CHUNK_SIZE as f32,
            );
            rec.check_collision_point_rec(position)
        })
    }
    pub fn get_tile_type_from_coords(&self, coords: Coords) -> TileType {
        let tile = &self.tiles.iter().find(|t| t.tile_coords == coords);

        if let Some(tile) = tile {
            tile.tile_type
        } else {
            TileType::Empty
        }
    }
}
