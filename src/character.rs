use raylib::ffi::Vector2;

use crate::obstacle::Obstacle;

pub enum CharacterType {
    Player,
    Enemy,
}

pub struct Character {
    pub character_type: CharacterType,
    pub position: Vector2,
    pub radius: f32,
    movement_speed: f32,
    is_traveling: bool,
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
            is_traveling: false,
        }
    }

    // FIX: Pathfinding doesn't work with obstacles connected to each other because
    // the waypoints might get placed inside other obstacles.
    // Current code only handles circle obstacles.
    pub fn set_travel_target_position(&mut self, target: Vector2, obstacles: &[Obstacle]) {
        if (target - self.position).length() >= Self::TRAVEL_THRESHOLD {
            let mut route: Vec<Vector2> = vec![];
            let mut sorted_obstacles: Vec<&Obstacle> = obstacles.iter().collect();
            sorted_obstacles.sort_by(|a, b| {
                (a.position - self.position)
                    .length()
                    .total_cmp(&(b.position - self.position).length())
            });
            let mut fixed_target = target;
            for obstacle in obstacles.iter() {
                while raylib::collision::check_collision_point_circle(
                    fixed_target,
                    obstacle.position,
                    obstacle.radius + 25.0,
                ) {
                    let push_direction = (fixed_target - obstacle.position).normalize();
                    fixed_target += push_direction;
                }
            }
            route.push(self.position);
            route.push(fixed_target);
            let mut checking_for_collision = true;
            let mut check_count = 0;
            while checking_for_collision {
                checking_for_collision = false;
                check_count += 1;

                let mut new_waypoints: Option<[Vector2; 2]> = None;
                let mut new_waypoint_index = 0;

                'waypoints: for (i, &waypoint) in route.iter().enumerate() {
                    if i == 0 {
                        continue;
                    };
                    let previous_point = route[i - 1];
                    sorted_obstacles.sort_by(|a, b| {
                        (a.position - previous_point)
                            .length()
                            .total_cmp(&(b.position - previous_point).length())
                    });
                    for obstacle in sorted_obstacles.iter() {
                        if raylib::collision::check_collision_circle_line(
                            obstacle.position,
                            obstacle.radius + self.radius,
                            previous_point,
                            waypoint,
                        ) {
                            const TANGENT_OFFSET: f32 = 24.0;
                            let from_tangents = tangent_points(
                                previous_point,
                                obstacle.position,
                                obstacle.radius + TANGENT_OFFSET,
                            );
                            let to_tangents = tangent_points(
                                waypoint,
                                obstacle.position,
                                obstacle.radius + TANGENT_OFFSET,
                            );

                            if let (Some(from_points), Some(to_points)) =
                                (from_tangents, to_tangents)
                            {
                                let left_side_length = waypoint.distance(to_points[0])
                                    + to_points[0].distance(from_points[1])
                                    + from_points[1].distance(previous_point);
                                let right_side_length = waypoint.distance(to_points[1])
                                    + to_points[1].distance(from_points[0])
                                    + from_points[0].distance(previous_point);
                                let left_points = [to_points[0], from_points[1]];
                                let right_points = [to_points[1], from_points[0]];
                                let mut points = if right_side_length < left_side_length {
                                    [right_points, left_points]
                                } else {
                                    [left_points, right_points]
                                };

                                if obstacles.iter().any(|o| {
                                    raylib::collision::check_collision_point_circle(
                                        points[0][0],
                                        o.position,
                                        o.radius + self.radius,
                                    ) || raylib::collision::check_collision_point_circle(
                                        points[0][1],
                                        o.position,
                                        o.radius + self.radius,
                                    )
                                }) {
                                    points.swap(0, 1);
                                }

                                new_waypoints = Some(points[0]);
                                new_waypoint_index = i;
                                break 'waypoints;
                            }
                        }
                    }
                }
                if let Some(points) = new_waypoints {
                    points.iter().for_each(|p| {
                        if !route.contains(&p) {
                            route.insert(new_waypoint_index, *p);
                            checking_for_collision = true;
                        } else {
                            println!("Route contains: {:?}", p);
                        }
                    });
                }
                if check_count > 50 {
                    break;
                }
            }

            self.path = route;
            self.is_traveling = true;
        }
    }

    pub fn move_character(&mut self, delta: f32) {
        if self.is_traveling {
            let &next_route_point = self.path.first().unwrap_or(&self.position);
            let difference = next_route_point - self.position;
            let direction = difference.normalize();
            self.position += direction * self.movement_speed * delta;
            if difference.length() <= Self::TRAVEL_THRESHOLD {
                self.path.remove(0);
                if self.path.is_empty() {
                    self.is_traveling = false;
                }
            }
        }
    }
}

fn tangent_points(point: Vector2, center: Vector2, radius: f32) -> Option<[Vector2; 2]> {
    let v = point - center;
    let d = v.length();

    // No tangent exists if the point is on or inside the clearance circle.
    if d <= radius {
        return None;
    }

    let u = v.normalize();
    let angle = (radius / d).acos();
    let (sin_angle, cos_angle) = angle.sin_cos();
    let perpendicular = Vector2::new(-u.y, u.x) * 1.1;

    Some([
        center + (u * cos_angle + perpendicular * sin_angle) * radius, // right side
        center + (u * cos_angle - perpendicular * sin_angle) * radius, // left side
    ])
}
