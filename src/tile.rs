use raylib::ffi::Vector2;

#[derive(Debug)]
pub struct Tile {
    pub center_position: Vector2,
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

#[derive(Debug)]
pub enum TileType {
    Empty,
    Obstacle,
}
