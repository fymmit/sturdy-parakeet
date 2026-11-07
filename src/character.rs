use raylib::math::Vector2;

use crate::chunks::ChunkManager;
use crate::pathfinding;
use crate::tile::Tile;

pub enum CharacterType {
    Player,
    Enemy,
}

pub struct Character {
    pub character_type: CharacterType,
    pub position: Vector2,
    pub radius: f32,
    movement_speed: f32,
    pub path: Vec<Vector2>,
}

impl Character {
    const TRAVEL_THRESHOLD: f32 = 5.0;

    pub fn new(character_type: CharacterType, position: Vector2, movement_speed: f32) -> Self {
        Character {
            character_type,
            position,
            radius: 16.0,
            movement_speed,
            path: vec![],
        }
    }

    pub fn set_target_position(
        &mut self,
        target_position: Vector2,
        chunk_manager: &ChunkManager,
        visible_tiles: &Vec<Tile>,
    ) {
        if let Some(route) = pathfinding::pathfind_to_target(
            self.position,
            target_position,
            chunk_manager,
            visible_tiles,
        ) {
            self.path = route;
        }
    }

    pub fn move_character(&mut self, delta: f32) {
        if !self.path.is_empty() {
            let &next_route_point = self.path.first().unwrap_or(&self.position);
            let difference = next_route_point - self.position;
            let direction = difference.normalize();
            self.position += direction * self.movement_speed * delta;
            if difference.length() <= Self::TRAVEL_THRESHOLD {
                self.path.remove(0);
            }
        }
    }
}
