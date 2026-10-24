use raylib::ffi::Vector2;

use crate::chunks::CHUNK_TILE_SIZE;

#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub global_x: i32,
    pub global_y: i32,
    pub tile_type: TileType,
}

impl Tile {
    pub fn get_center(&self) -> Vector2 {
        let half_tile_size = CHUNK_TILE_SIZE / 2;
        let x = (self.global_x + half_tile_size) as f32;
        let y = (self.global_y + half_tile_size) as f32;
        Vector2::new(x, y)
    }
}

#[derive(Debug)]
pub enum TileType {
    Empty,
    Obstacle,
}
