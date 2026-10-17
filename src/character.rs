use raylib::ffi::Vector2;

pub enum CharacterType {
    Player,
    Enemy,
}

pub struct Character {
    pub character_type: CharacterType,
    pub position: Vector2,
    movement_speed: f32,
    travel_target_position: Vector2,
    is_traveling: bool,
}

impl Character {
    const TRAVEL_THRESHOLD: f32 = 5.0;

    pub fn new(character_type: CharacterType, position: Vector2, movement_speed: f32) -> Self {
        Character {
            character_type,
            position,
            movement_speed,
            travel_target_position: Vector2::zero(),
            is_traveling: false,
        }
    }

    pub fn set_travel_target_position(&mut self, target: Vector2) {
        if (target - self.position).length() >= Self::TRAVEL_THRESHOLD {
            self.travel_target_position = target;
            self.is_traveling = true;
        }
    }

    pub fn move_character(&mut self, delta: f32) {
        if self.is_traveling {
            let difference = self.travel_target_position - self.position;
            let direction = difference.normalize();
            self.position += direction * self.movement_speed * delta;
            if difference.length() <= Self::TRAVEL_THRESHOLD {
                self.is_traveling = false;
            }
        }
    }
}
