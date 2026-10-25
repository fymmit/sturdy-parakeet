use raylib::ffi::Vector2;

use crate::{
    chunks::{CHUNK_TILE_SIZE, ChunkManager},
    tile::{Coords, Tile, TileType},
};

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

    pub fn pathfind_to_target(&mut self, target: Vector2, chunk_manager: &ChunkManager) {
        let self_coords = Tile::get_coords_from_position(self.position);
        let target_coords = Tile::get_coords_from_position(target);
        if chunk_manager.get_tile_type_from_coords(target_coords) == TileType::Obstacle {
            return;
        }

        let route = self.build_route(self_coords, target_coords, chunk_manager);
        self.path = route;
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

    fn build_route(
        &self,
        self_coords: Coords,
        target_coords: Coords,
        chunk_manager: &ChunkManager,
    ) -> Vec<Vector2> {
        let mut possible_routes: Vec<Vec<Vector2>> = vec![];

        for i in 0..4 {
            let mut route: Vec<Coords> = vec![];
            let mut bad_coords: Vec<Coords> = vec![];

            route.push(target_coords);

            let mut current_coords = target_coords;
            let mut loop_count = 0;
            loop {
                loop_count += 1;
                if loop_count >= 100 {
                    println!("infinite loop");
                    route = vec![];
                    break;
                }

                let x_diff = self_coords.x - current_coords.x;
                let y_diff = self_coords.y - current_coords.y;
                let mut x_direction = x_diff.signum();
                let mut y_direction = y_diff.signum();

                if x_direction.abs() == y_direction.abs() {
                    if i < 2 {
                        if x_diff < y_diff {
                            y_direction = 0;
                        } else {
                            x_direction = 0;
                        }
                    } else {
                        if x_diff > y_diff {
                            y_direction = 0;
                        } else {
                            x_direction = 0;
                        }
                    }
                }

                let mut candidate_coords = Coords {
                    x: current_coords.x + x_direction,
                    y: current_coords.y + y_direction,
                };

                if candidate_coords == self_coords {
                    println!("path done");
                    break;
                }

                let mut previous_coords: Option<Coords> = None;
                if route.len() >= 2 {
                    previous_coords = Some(route[route.len() - 2]);
                }

                let mut is_bad_coords = true;
                for _ in 0..4 {
                    let candidate_already_on_route = if let Some(prev) = previous_coords {
                        candidate_coords == prev || bad_coords.contains(&candidate_coords)
                    } else {
                        false
                    };
                    let tile_type = chunk_manager.get_tile_type_from_coords(candidate_coords);
                    let tile_type_is_obstacle = tile_type == TileType::Obstacle;
                    if tile_type_is_obstacle || candidate_already_on_route {
                        if i % 2 == 0 {
                            let temp = y_direction;
                            y_direction = -x_direction;
                            x_direction = temp;
                        } else {
                            let temp = x_direction;
                            x_direction = -y_direction;
                            y_direction = temp;
                        }
                        candidate_coords = Coords {
                            x: current_coords.x + x_direction,
                            y: current_coords.y + y_direction,
                        };
                        continue;
                    }
                    is_bad_coords = false;
                    break;
                }

                if is_bad_coords {
                    bad_coords.push(current_coords);
                    route.pop();
                } else {
                    println!("pushing to route: {:?}", candidate_coords);
                    route.push(candidate_coords);
                    current_coords = candidate_coords;
                }
            }
            let mut route: Vec<Vector2> = route
                .iter()
                .map(|c| Tile::get_center_from_coords(*c))
                .collect();
            route.push(self.position);
            route.reverse();

            if !route.is_empty() {
                let mut necessary_waypoints: Vec<Vector2> = vec![];
                let mut current_necessary_index = 0;
                necessary_waypoints.push(route[current_necessary_index]);

                let obstacles: Vec<&Tile> = chunk_manager
                    .tiles
                    .iter()
                    .filter(|t| t.tile_type == TileType::Obstacle)
                    .collect();

                for foo in 0..route.len() {
                    if foo <= current_necessary_index {
                        continue;
                    }
                    let mut largest_visible_index = 0;
                    for i in current_necessary_index..route.len() {
                        let current = route[current_necessary_index];
                        let next = route[i];
                        let mut can_see = true;
                        for &obstacle in obstacles.iter() {
                            let center = Tile::get_center_from_coords(obstacle.tile_coords);
                            let radius = (CHUNK_TILE_SIZE as f32 / 2.0) * 1.8;
                            if raylib::check_collision_circle_line(center, radius, current, next) {
                                can_see = false;
                                break;
                            }
                        }
                        if can_see {
                            largest_visible_index = i;
                        }
                    }
                    current_necessary_index = largest_visible_index;
                    necessary_waypoints.push(route[largest_visible_index]);
                }
                necessary_waypoints.push(*route.last().unwrap());
                route = necessary_waypoints;
            }

            possible_routes.push(route);
        }

        possible_routes.sort_by(|a, b| get_route_length(a).total_cmp(&get_route_length(b)));

        let shortest_route = possible_routes
            .iter()
            .find(|r| get_route_length(r) > 0.0)
            .unwrap_or(&vec![])
            .clone();
        shortest_route
    }
}

fn get_route_length(route: &Vec<Vector2>) -> f32 {
    let mut route_length: f32 = 0.0;

    for (i, waypoint) in route.iter().enumerate() {
        if i == 0 {
            continue;
        };
        let prev = route[i - 1];
        let distance = waypoint.distance(prev);
        route_length += distance;
    }
    println!("Route length: {}", route_length);
    route_length
}
