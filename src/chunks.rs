use rand::prelude::*;
use rand::rngs::SmallRng;
use raylib::ffi::{Rectangle, Vector2};

use crate::tile::{Tile, TileType};

pub const CHUNK_TILE_COUNT: i32 = 16;
pub const CHUNK_TILE_SIZE: i32 = 64;
pub const CHUNK_SIZE: i32 = CHUNK_TILE_COUNT * CHUNK_TILE_SIZE;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub tiles: Vec<Tile>,
}

pub struct ChunkManager {
    pub chunks: Vec<Chunk>,
    rng: SmallRng,
}

impl ChunkManager {
    pub fn new() -> Self {
        let bytes: Vec<u8> = "foobar".bytes().collect();
        let mut seed: u64 = 0;
        for b in bytes.iter() {
            seed += *b as u64;
        }
        let rng = SmallRng::seed_from_u64(seed);
        Self {
            chunks: vec![],
            rng,
        }
    }
    pub fn generate_chunk(&mut self, x_coord: i32, y_coord: i32) {
        println!("Generating a chunk at: {}, {}", x_coord, y_coord);
        let mut tiles: Vec<Tile> = vec![];
        for x in 0..CHUNK_TILE_COUNT {
            for y in 0..CHUNK_TILE_COUNT {
                let tile_should_be_obstacle = self.rng.random_range(0..10) == 0;
                let tile_type = if tile_should_be_obstacle {
                    TileType::Obstacle
                } else {
                    TileType::Empty
                };
                tiles.push(Tile { x, y, tile_type });
            }
        }

        self.chunks.push(Chunk {
            x: x_coord,
            y: y_coord,
            tiles,
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
}
