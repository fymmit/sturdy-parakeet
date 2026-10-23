pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

pub enum TileType {
    Empty,
    Obstacle,
}
