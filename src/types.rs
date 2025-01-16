use glam::Vec2;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(pub Uuid);

#[derive(Debug, Clone)]
pub struct Robot {
    pub id: PlayerId,
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub team: Team,
    pub has_ball: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Blue,
    Yellow,
}

#[derive(Debug, Clone)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Debug, Clone)]
pub struct WorldState {
    pub robots: Vec<Robot>,
    pub ball: Ball,
    pub field: Field,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub width: f32,
    pub height: f32,
    pub goal_width: f32,
}

impl Field {
    pub fn new() -> Self {
        Self {
            width: 9000.0,      // 9 meters
            height: 6000.0,     // 6 meters
            goal_width: 1000.0, // 1 meter
        }
    }
}
