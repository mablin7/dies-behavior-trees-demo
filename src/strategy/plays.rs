use super::behavior::*;
use super::situation::*;
use crate::types::{PlayerId, Team};
use glam::Vec2;

// Common situations
pub fn has_ball() -> Situation<impl Fn(&RobotSituation) -> bool> {
    Situation::new(|s| s.has_ball, "has_ball")
}

pub fn good_shot_opportunity() -> Situation<impl Fn(&RobotSituation) -> bool> {
    Situation::new(
        |s| s.shot_quality > 0.7 && !s.is_marked,
        "good_shot_opportunity",
    )
    .with_visualization(DebugVisualization::Circle {
        radius: 1000.0,
        color: DebugColor {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 0.5,
        },
    })
}

pub fn teammate_has_ball() -> Situation<impl Fn(&RobotSituation) -> bool> {
    Situation::new(
        |s| s.teammates_positions.iter().any(|(_, _)| true),
        "teammate_has_ball",
    )
}

// Striker role - primary scoring threat
pub fn create_striker(player_id: PlayerId) -> impl BehaviorNode {
    let mut select = select();

    // When we have the ball and a good shot, take it
    let shoot_sequence = {
        let mut seq = sequence();
        seq.children.push(Box::new(ShootAction {
            min_shot_quality: 0.7,
        }));
        seq
    };

    // When teammate has ball, find good scoring position
    let support_sequence = {
        let mut seq = sequence();
        seq.children.push(Box::new(MoveToAction {
            target: Vec2::new(1000.0, 0.0),
            tolerance: 100.0,
        }));
        seq
    };

    // Add sequences to select
    select.children.push(Box::new(shoot_sequence));
    select.children.push(Box::new(support_sequence));

    select
}

// Support attacker - helps create scoring opportunities
pub fn create_support_attacker(player_id: PlayerId) -> impl BehaviorNode {
    let mut select = select();

    // When striker has ball, move to support position
    let support_sequence = {
        let mut seq = sequence();
        seq.children.push(Box::new(MoveToAction {
            target: Vec2::new(-500.0, 500.0), // Support position
            tolerance: 100.0,
        }));
        seq
    };

    // When we have ball, look for pass to striker
    let pass_sequence = {
        let mut seq = sequence();
        seq.children.push(Box::new(MoveToAction {
            target: Vec2::new(0.0, 0.0),
            tolerance: 100.0,
        }));
        seq
    };

    select.children.push(Box::new(support_sequence));
    select.children.push(Box::new(pass_sequence));
    select
}

// Team coordinator - manages high-level strategy
pub struct TeamStrategy {
    striker_id: PlayerId,
    support_id: PlayerId,
    striker: Box<dyn BehaviorNode>,
    support: Box<dyn BehaviorNode>,
}

impl TeamStrategy {
    pub fn new(striker_id: PlayerId, support_id: PlayerId) -> Self {
        Self {
            striker_id,
            support_id,
            striker: Box::new(create_striker(striker_id)),
            support: Box::new(create_support_attacker(support_id)),
        }
    }

    pub fn update(&mut self, striker_situation: RobotSituation, support_situation: RobotSituation) {
        let mut striker_ctx = BehaviorContext {
            robot_id: self.striker_id,
            situation: striker_situation,
        };

        let mut support_ctx = BehaviorContext {
            robot_id: self.support_id,
            situation: support_situation,
        };

        // Update both behaviors
        self.striker.tick(&mut striker_ctx);
        self.support.tick(&mut support_ctx);
    }
}
