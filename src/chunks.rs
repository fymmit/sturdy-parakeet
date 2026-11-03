use noise::{NoiseFn, Perlin};
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
    // FIX: use HashMap for tiles, should be far better
    pub tiles: Vec<Tile>,
    perlin: Perlin,
    chunks: Vec<Chunk>,
    seed: String,
}

impl ChunkManager {
    pub fn new(seed_str: String) -> Self {
        Self {
            tiles: vec![],
            chunks: vec![],
            seed: seed_str,
            perlin: Perlin::new(5),
        }
    }
    pub fn generate_chunk(&mut self, position: Vector2) {
        let chunk_x = (position.x / CHUNK_SIZE as f32).floor() as i32;
        let x_coord = chunk_x * CHUNK_SIZE;
        let chunk_y = (position.y / CHUNK_SIZE as f32).floor() as i32;
        let y_coord = chunk_y * CHUNK_SIZE;
        let chunk_seed = format!("{}{}{}", self.seed, chunk_x, chunk_y);
        let bytes: Vec<u8> = chunk_seed.bytes().collect();
        let mut seed: u64 = 0;
        for b in bytes.iter() {
            seed += *b as u64;
        }
        let mut rng = SmallRng::seed_from_u64(seed);
        println!("Generating a chunk at: {}, {}", chunk_x, chunk_y);
        let mut tiles: Vec<Tile> = vec![];
        for x in 0..CHUNK_TILE_COUNT {
            for y in 0..CHUNK_TILE_COUNT {
                let x_position = x_coord + x * CHUNK_TILE_SIZE;
                let y_position = y_coord + y * CHUNK_TILE_SIZE;

                let coefficient = 0.001;
                let noise_x = x_position as f64 * coefficient;
                let noise_y = y_position as f64 * coefficient;
                let noise_value = self.perlin.get([noise_x, noise_y]);

                let x = chunk_x * CHUNK_TILE_COUNT + x;
                let y = chunk_y * CHUNK_TILE_COUNT + y;

                let tile_type = if noise_value > 0.3 {
                    TileType::Water
                } else {
                    TileType::Empty
                };
                let coords = Coords { x, y };
                tiles.push(Tile {
                    tile_coords: coords,
                    x_position,
                    y_position,
                    tile_type,
                });
            }
        }

        for tile in tiles.iter_mut() {
            if tile.tile_type == TileType::Empty {
                let tile_random_value = rng.random_range(0..20);
                if tile_random_value == 0 {
                    tile.tile_type = TileType::Obstacle;
                }
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
