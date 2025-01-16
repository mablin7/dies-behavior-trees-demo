use crate::types::{PlayerId, Team, WorldState};
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct RobotSituation {
    pub position: Vec2,
    pub velocity: Vec2,
    pub has_ball: bool,
    pub team: Team,
    pub shot_quality: f32,
    pub is_marked: bool,
    pub teammates_positions: Vec<(PlayerId, Vec2)>,
    pub opponents_positions: Vec<(PlayerId, Vec2)>,
}

#[derive(Debug, Clone)]
pub struct Situation<F>
where
    F: Fn(&RobotSituation) -> bool,
{
    condition: F,
    description: String,
    visualization: Option<DebugVisualization>,
}

#[derive(Debug, Clone)]
pub enum DebugVisualization {
    Circle {
        radius: f32,
        color: DebugColor,
    },
    Line {
        start: Vec2,
        end: Vec2,
        color: DebugColor,
    },
    Text {
        position: Vec2,
        content: String,
        color: DebugColor,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct DebugColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<F> Situation<F>
where
    F: Fn(&RobotSituation) -> bool,
{
    pub fn new(condition: F, description: impl Into<String>) -> Self {
        Self {
            condition,
            description: description.into(),
            visualization: None,
        }
    }

    pub fn with_visualization(mut self, vis: DebugVisualization) -> Self {
        self.visualization = Some(vis);
        self
    }

    pub fn evaluate(&self, situation: &RobotSituation) -> bool {
        (self.condition)(situation)
    }

    pub fn and<G>(self, other: Situation<G>) -> Situation<impl Fn(&RobotSituation) -> bool>
    where
        G: Fn(&RobotSituation) -> bool,
    {
        Situation::new(
            move |s| (self.condition)(s) && (other.condition)(s),
            format!("({} AND {})", self.description, other.description),
        )
    }

    pub fn or<G>(self, other: Situation<G>) -> Situation<impl Fn(&RobotSituation) -> bool>
    where
        G: Fn(&RobotSituation) -> bool,
    {
        Situation::new(
            move |s| (self.condition)(s) || (other.condition)(s),
            format!("({} OR {})", self.description, other.description),
        )
    }
}
