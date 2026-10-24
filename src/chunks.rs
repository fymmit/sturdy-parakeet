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
    pub fn new(seed_str: &str) -> Self {
        let bytes: Vec<u8> = seed_str.bytes().collect();
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
    pub fn generate_chunk(&mut self, position: Vector2) {
        let x_coord = (position.x / CHUNK_SIZE as f32).floor() as i32 * CHUNK_SIZE;
        let y_coord = (position.y / CHUNK_SIZE as f32).floor() as i32 * CHUNK_SIZE;
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
                let global_x = x_coord + x * CHUNK_TILE_SIZE;
                let global_y = y_coord + y * CHUNK_TILE_SIZE;
                let half_tile_size = CHUNK_TILE_SIZE / 2;
                let center_x = (global_x + half_tile_size) as f32;
                let center_y = (global_y + half_tile_size) as f32;
                let center_position = Vector2::new(center_x, center_y);
                tiles.push(Tile {
                    center_position,
                    x: global_x,
                    y: global_y,
                    tile_type,
                });
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
    pub fn get_tile_center(&self, position: Vector2) -> Vector2 {
        let chunk = self
            .chunks
            .iter()
            .find(|c| {
                let rec =
                    Rectangle::new(c.x as f32, c.y as f32, CHUNK_SIZE as f32, CHUNK_SIZE as f32);
                rec.check_collision_point_rec(position)
            })
            .expect("chunk should be generated");
        let tile = chunk
            .tiles
            .iter()
            .find(|t| {
                let rec = Rectangle::new(
                    t.x as f32,
                    t.y as f32,
                    CHUNK_TILE_SIZE as f32,
                    CHUNK_TILE_SIZE as f32,
                );
                rec.check_collision_point_rec(position)
            })
            .expect("tile should exist");

        tile.center_position
    }
}
