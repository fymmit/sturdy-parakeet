use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, raylib-rs")
        .build();

    rl.set_target_fps(150);

    let mut characters: Vec<Character> = vec![];

    let player = Character {
        character_type: CharacterType::Player,
        position: Vector2::new(
            (rl.get_render_width() / 2) as f32,
            (rl.get_render_height() / 2) as f32,
        ),
        movement_speed: 200.0,
        target_position: Vector2::zero(),
        is_traveling: false,
    };

    let enemy = Character {
        character_type: CharacterType::Enemy,
        position: Vector2::new(100.0, 100.0),
        movement_speed: 100.0,
        target_position: Vector2::zero(),
        is_traveling: false,
    };

    characters.push(player);
    characters.push(enemy);

    while !rl.window_should_close() {
        update(&rl, &mut characters);

        let mut d = rl.begin_drawing(&thread);

        draw(&mut d, &characters);
    }
}

fn update(rl: &RaylibHandle, characters: &mut [Character]) {
    let delta = rl.get_frame_time();
    let mouse_pos = rl.get_mouse_position();
    let player_pos = characters.first().unwrap().position;

    for character in characters.iter_mut() {
        match character.character_type {
            CharacterType::Player => {
                if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                    character.set_target_position(mouse_pos);
                }
            }
            CharacterType::Enemy => {
                character.set_target_position(player_pos);
            }
        }
        character.move_character(delta);
    }
}

fn draw(d: &mut RaylibDrawHandle, characters: &[Character]) {
    d.clear_background(Color::WHITE);
    let mut ordered_characters: Vec<&Character> = characters.into_iter().collect();
    ordered_characters.sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    for character in ordered_characters.iter() {
        let color = match character.character_type {
            CharacterType::Player => Color::GREEN,
            CharacterType::Enemy => Color::RED,
        };
        d.draw_circle(
            character.position.x as i32,
            character.position.y as i32,
            16.0,
            color,
        );
    }

    d.draw_fps(4, 4);
}

enum CharacterType {
    Player,
    Enemy,
}

struct Character {
    character_type: CharacterType,
    position: Vector2,
    movement_speed: f32,
    target_position: Vector2,
    is_traveling: bool,
}

impl Character {
    const TRAVEL_THRESHOLD: f32 = 5.0;

    fn set_target_position(&mut self, target: Vector2) {
        if (target - self.position).length() >= Self::TRAVEL_THRESHOLD {
            self.target_position = target;
            self.is_traveling = true;
        }
    }

    fn move_character(&mut self, delta: f32) {
        if self.is_traveling {
            let difference = self.target_position - self.position;
            let direction = difference.normalize();
            self.position += direction * self.movement_speed * delta;
            if difference.length() <= Self::TRAVEL_THRESHOLD {
                self.is_traveling = false;
            }
        }
    }
}
