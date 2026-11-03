use raylib::math::Vector2;

use crate::chunks::CHUNK_TILE_SIZE;

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_coords: Coords,
    pub global_position_x: i32,
    pub global_position_y: i32,
    pub tile_type: TileType,
}

impl Tile {
    pub fn get_center_from_coords(coords: Coords) -> Vector2 {
        let half_tile_size = CHUNK_TILE_SIZE / 2;
        let x = (coords.x * CHUNK_TILE_SIZE + half_tile_size) as f32;
        let y = (coords.y * CHUNK_TILE_SIZE + half_tile_size) as f32;
        Vector2::new(x, y)
    }
    pub fn get_coords_from_position(position: Vector2) -> Coords {
        let x = (position.x / CHUNK_TILE_SIZE as f32).floor() as i32;
        let y = (position.y / CHUNK_TILE_SIZE as f32).floor() as i32;
        Coords { x, y }
    }
    pub fn get_center_from_position(position: Vector2) -> Vector2 {
        let coords = Self::get_coords_from_position(position);
        Self::get_center_from_coords(coords)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum TileType {
    Empty,
    Obstacle,
    Water,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}
