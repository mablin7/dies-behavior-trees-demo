use super::*;
use glam::Vec2;

pub struct MoveToAction {
    pub target: Vec2,
    pub tolerance: f32,
}

impl BehaviorNode for MoveToAction {
    fn tick(&mut self, ctx: &mut BehaviorContext) -> NodeResult {
        let distance = (self.target - ctx.situation.position).length();

        if distance <= self.tolerance {
            NodeResult::Success
        } else {
            // Use the robot control API to move towards target
            // This is where we'll integrate with your existing RobotControl
            NodeResult::Running
        }
    }
}

pub struct ShootAction {
    pub min_shot_quality: f32,
}

impl BehaviorNode for ShootAction {
    fn tick(&mut self, ctx: &mut BehaviorContext) -> NodeResult {
        if ctx.situation.shot_quality >= self.min_shot_quality {
            // Implement shooting logic using robot control
            NodeResult::Success
        } else {
            NodeResult::Failure("Shot quality too low".into())
        }
    }
}
