mod game_engine;
mod robot_control;
mod strategy;
mod types;

use game_engine::GameEngine;
use ggez::{conf, event, ContextBuilder};
use glam::Vec2;
use robot_control::RobotControl;
use strategy::{plays::TeamStrategy, RobotSituation};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("soccer-strategy-sandbox", "author")
        .window_setup(conf::WindowSetup::default().title("Soccer Strategy Sandbox"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 800.0))
        .build()
        .expect("Failed to create context");

    let mut game = GameEngine::new();

    // Get two blue team robots for our strategy
    let blue_robots = game.get_team_robots(types::Team::Blue);
    if blue_robots.len() >= 2 {
        let striker_id = blue_robots[0];
        let support_id = blue_robots[1];

        // Create our team strategy
        let mut strategy = TeamStrategy::new(striker_id, support_id);

        // Example of updating the strategy (this would normally happen in the game loop)
        let striker_situation = RobotSituation {
            position: Vec2::new(-2000.0, 0.0),
            velocity: Vec2::ZERO,
            has_ball: true,
            team: types::Team::Blue,
            shot_quality: 0.8,
            is_marked: false,
            teammates_positions: vec![(support_id, Vec2::new(-1500.0, 500.0))],
            opponents_positions: vec![],
        };

        let support_situation = RobotSituation {
            position: Vec2::new(-1500.0, 500.0),
            velocity: Vec2::ZERO,
            has_ball: false,
            team: types::Team::Blue,
            shot_quality: 0.0,
            is_marked: false,
            teammates_positions: vec![(striker_id, Vec2::new(-2000.0, 0.0))],
            opponents_positions: vec![],
        };

        strategy.update(striker_situation, support_situation);
    }

    event::run(ctx, event_loop, game);
}
