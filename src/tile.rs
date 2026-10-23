#[derive(Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

#[derive(Debug)]
pub enum TileType {
    Empty,
    Obstacle,
}
