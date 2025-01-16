use crate::game_engine::GameEngine;
use crate::types::{PlayerId, Team};
use glam::Vec2;

pub struct RobotControl<'a> {
    engine: &'a mut GameEngine,
}

impl<'a> RobotControl<'a> {
    pub fn new(engine: &'a mut GameEngine) -> Self {
        Self { engine }
    }

    pub fn get_team_robots(&self, team: Team) -> Vec<PlayerId> {
        self.engine.get_team_robots(team)
    }

    pub fn set_robot_velocity(&mut self, id: PlayerId, velocity: Vec2) {
        self.engine.set_robot_velocity(id, velocity);
    }

    pub fn set_robot_rotation(&mut self, id: PlayerId, rotation: f32) {
        self.engine.set_robot_rotation(id, rotation);
    }

    // Helper method to move robot towards a target position
    pub fn move_robot_to(&mut self, id: PlayerId, target: Vec2, max_speed: f32) {
        let current_pos = self.engine.get_robot_position(id);
        if let Some(pos) = current_pos {
            let direction = target - pos;
            let distance = direction.length();

            if distance > 1.0 {
                // Avoid jitter when close to target
                let velocity = direction.normalize() * max_speed.min(distance);
                self.set_robot_velocity(id, velocity);
            } else {
                self.set_robot_velocity(id, Vec2::ZERO);
            }
        }
    }
}
