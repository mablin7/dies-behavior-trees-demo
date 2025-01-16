use ggez::{
    event,
    graphics::{self, Canvas, Color},
    Context, GameResult,
};
use glam::Vec2;

use crate::types::*;

pub struct GameEngine {
    world_state: WorldState,
    pixels_per_meter: f32,
}

impl GameEngine {
    pub fn new() -> Self {
        let field = Field::new();
        let world_state = WorldState {
            robots: vec![
                // Blue team
                Robot {
                    id: PlayerId(uuid::Uuid::new_v4()),
                    position: Vec2::new(-2000.0, -1000.0),
                    velocity: Vec2::ZERO,
                    rotation: 0.0,
                    team: Team::Blue,
                    has_ball: false,
                },
                Robot {
                    id: PlayerId(uuid::Uuid::new_v4()),
                    position: Vec2::new(-2000.0, 0.0),
                    velocity: Vec2::ZERO,
                    rotation: 0.0,
                    team: Team::Blue,
                    has_ball: false,
                },
                Robot {
                    id: PlayerId(uuid::Uuid::new_v4()),
                    position: Vec2::new(-2000.0, 1000.0),
                    velocity: Vec2::ZERO,
                    rotation: 0.0,
                    team: Team::Blue,
                    has_ball: false,
                },
                // Yellow team
                Robot {
                    id: PlayerId(uuid::Uuid::new_v4()),
                    position: Vec2::new(2000.0, -1000.0),
                    velocity: Vec2::ZERO,
                    rotation: 0.0,
                    team: Team::Yellow,
                    has_ball: false,
                },
                Robot {
                    id: PlayerId(uuid::Uuid::new_v4()),
                    position: Vec2::new(2000.0, 0.0),
                    velocity: Vec2::ZERO,
                    rotation: 0.0,
                    team: Team::Yellow,
                    has_ball: false,
                },
                Robot {
                    id: PlayerId(uuid::Uuid::new_v4()),
                    position: Vec2::new(2000.0, 1000.0),
                    velocity: Vec2::ZERO,
                    rotation: 0.0,
                    team: Team::Yellow,
                    has_ball: false,
                },
            ],
            ball: Ball {
                position: Vec2::ZERO,
                velocity: Vec2::ZERO,
            },
            field,
        };

        Self {
            world_state,
            pixels_per_meter: 100.0,
        }
    }

    pub fn set_robot_velocity(&mut self, id: PlayerId, velocity: Vec2) {
        if let Some(robot) = self.world_state.robots.iter_mut().find(|r| r.id == id) {
            robot.velocity = velocity;
        }
    }

    pub fn set_robot_rotation(&mut self, id: PlayerId, rotation: f32) {
        if let Some(robot) = self.world_state.robots.iter_mut().find(|r| r.id == id) {
            robot.rotation = rotation;
        }
    }

    pub fn get_team_robots(&self, team: Team) -> Vec<PlayerId> {
        self.world_state
            .robots
            .iter()
            .filter(|r| r.team == team)
            .map(|r| r.id)
            .collect()
    }

    fn update_physics(&mut self, dt: f32) {
        for robot in &mut self.world_state.robots {
            robot.position += robot.velocity * dt;
        }

        self.world_state.ball.position += self.world_state.ball.velocity * dt;
    }

    fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let screen_pos = world_pos * self.pixels_per_meter;
        Vec2::new(
            screen_pos.x + 600.0, // Center horizontally (half of 1200)
            screen_pos.y + 400.0, // Center vertically (half of 800)
        )
    }

    fn draw_field(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let field_rect = graphics::Rect::new(
            400.0 - (self.world_state.field.width * self.pixels_per_meter / 2.0),
            300.0 - (self.world_state.field.height * self.pixels_per_meter / 2.0),
            self.world_state.field.width * self.pixels_per_meter,
            self.world_state.field.height * self.pixels_per_meter,
        );

        let field_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            field_rect,
            Color::WHITE,
        )?;

        canvas.draw(&field_mesh, graphics::DrawParam::default());

        let center_line = graphics::Mesh::new_line(
            ctx,
            &[
                [
                    400.0,
                    300.0 - (self.world_state.field.height * self.pixels_per_meter / 2.0),
                ],
                [
                    400.0,
                    300.0 + (self.world_state.field.height * self.pixels_per_meter / 2.0),
                ],
            ],
            2.0,
            Color::WHITE,
        )?;

        canvas.draw(&center_line, graphics::DrawParam::default());
        Ok(())
    }

    fn draw_robots(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for robot in &self.world_state.robots {
            let screen_pos = self.world_to_screen(robot.position);
            let robot_size = 50.0; // Larger robots
            let robot_circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [screen_pos.x, screen_pos.y],
                robot_size,
                0.1,
                match robot.team {
                    Team::Blue => Color::BLUE,
                    Team::Yellow => Color::YELLOW,
                },
            )?;
            canvas.draw(&robot_circle, graphics::DrawParam::default());
        }
        Ok(())
    }

    fn draw_ball(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let screen_pos = self.world_to_screen(self.world_state.ball.position);
        let ball_circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [screen_pos.x, screen_pos.y],
            10.0,
            0.1,
            Color::WHITE,
        )?;
        canvas.draw(&ball_circle, graphics::DrawParam::default());
        Ok(())
    }

    pub fn get_robot_position(&self, id: PlayerId) -> Option<Vec2> {
        self.world_state
            .robots
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.position)
    }
}

impl event::EventHandler<ggez::GameError> for GameEngine {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        const DT: f32 = 1.0 / 60.0; // 60 FPS physics update
        self.update_physics(DT);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.1, 0.5, 0.1, 1.0));
        self.draw_field(ctx, &mut canvas)?;
        self.draw_robots(ctx, &mut canvas)?;
        self.draw_ball(ctx, &mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }
}
