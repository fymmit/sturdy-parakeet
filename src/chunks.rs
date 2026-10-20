use crate::tile::Tile;

pub struct Chunk {
    pub tiles: Vec<Tile>,
}

pub struct ChunkManager {
    pub chunks: Vec<Chunk>,
}

impl ChunkManager {
    pub fn new() -> Self {
        Self { chunks: vec![] }
    }
    pub fn generate_chunk(&mut self) {
        println!("Generating a chunk");
        let mut tiles: Vec<Tile> = vec![];
        for x in 0..16 {
            for y in 0..16 {
                tiles.push(Tile { x, y });
            }
        }
        self.chunks.push(Chunk { tiles });
    }
}
