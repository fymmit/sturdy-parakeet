use raylib::ffi::{Rectangle, Vector2};

use crate::tile::Tile;

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
}

impl ChunkManager {
    pub fn new() -> Self {
        Self { chunks: vec![] }
    }
    pub fn generate_chunk(&mut self, x_coord: i32, y_coord: i32) {
        println!("Generating a chunk at: {}, {}", x_coord, y_coord);
        let mut tiles: Vec<Tile> = vec![];
        for x in 0..CHUNK_TILE_COUNT {
            for y in 0..CHUNK_TILE_COUNT {
                tiles.push(Tile { x, y });
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
