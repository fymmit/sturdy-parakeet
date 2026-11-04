use std::collections::HashMap;

use noise::{NoiseFn, Perlin};
use raylib::math::{Rectangle, Vector2};

use crate::tile::{Coords, Tile, TileType};

pub const CHUNK_TILE_COUNT: i32 = 16;
pub const CHUNK_TILE_SIZE: i32 = 64;
pub const CHUNK_SIZE: i32 = CHUNK_TILE_COUNT * CHUNK_TILE_SIZE;

const NOISE_COEFFICIENT: f64 = 0.1;

pub struct Chunk {
    pub coords: Coords,
    pub tiles: Vec<Tile>,
}

pub struct ChunkManager {
    pub tiles: HashMap<Coords, Tile>,
    perlin: Perlin,
    chunks: Vec<Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            chunks: vec![],
            perlin: Perlin::new(5),
        }
    }

    pub fn generate_chunk(&mut self, position: Vector2) {
        let chunk_x = (position.x / CHUNK_SIZE as f32).floor() as i32;
        let x_coord = chunk_x * CHUNK_SIZE;
        let chunk_y = (position.y / CHUNK_SIZE as f32).floor() as i32;
        let y_coord = chunk_y * CHUNK_SIZE;
        let mut tiles: Vec<Tile> = vec![];
        for x in 0..CHUNK_TILE_COUNT {
            for y in 0..CHUNK_TILE_COUNT {
                let global_position_x = x_coord + x * CHUNK_TILE_SIZE;
                let global_position_y = y_coord + y * CHUNK_TILE_SIZE;
                let x = chunk_x * CHUNK_TILE_COUNT + x;
                let y = chunk_y * CHUNK_TILE_COUNT + y;

                let noise_x = x as f64 * NOISE_COEFFICIENT;
                let noise_y = y as f64 * NOISE_COEFFICIENT;
                let noise_value = self.perlin.get([noise_x, noise_y]);

                let tile_type = match noise_value {
                    ..-0.3 => TileType::Water,
                    0.7.. => TileType::Obstacle,
                    _ => TileType::Empty,
                };

                let coords = Coords { x, y };
                tiles.push(Tile {
                    tile_coords: coords,
                    global_position_x,
                    global_position_y,
                    tile_type,
                });
            }
        }

        for tile in tiles.iter() {
            self.tiles.insert(tile.tile_coords, tile.clone());
        }

        self.chunks.push(Chunk {
            coords: Coords {
                x: x_coord,
                y: y_coord,
            },
            tiles,
        });
    }

    pub fn is_chunk_generated(&self, position: Vector2) -> bool {
        self.chunks.iter().any(|chunk| {
            let rec = Rectangle::new(
                chunk.coords.x as f32,
                chunk.coords.y as f32,
                CHUNK_SIZE as f32,
                CHUNK_SIZE as f32,
            );
            rec.check_collision_point_rec(position)
        })
    }

    pub fn get_tiles_from_chunks_around_position(&self, position: Vector2) -> Vec<Tile> {
        let chunk_size = CHUNK_SIZE as f32;
        let position_rec = Rectangle {
            x: position.x - chunk_size,
            y: position.y - chunk_size,
            width: chunk_size * 2.0,
            height: chunk_size * 2.0,
        };
        let tiles: Vec<Tile> = self
            .chunks
            .iter()
            .filter(|chunk| {
                let rec = Rectangle::new(
                    chunk.coords.x as f32,
                    chunk.coords.y as f32,
                    CHUNK_SIZE as f32,
                    CHUNK_SIZE as f32,
                );
                rec.check_collision_recs(position_rec)
            })
            .flat_map(|chunk| chunk.tiles.clone())
            .collect();
        tiles
    }

    pub fn get_tile_type_from_coords(&self, coords: Coords) -> TileType {
        let tile = &self.tiles.get(&coords);

        if let Some(tile) = tile {
            tile.tile_type
        } else {
            TileType::Empty
        }
    }
}
