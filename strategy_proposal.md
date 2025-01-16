# Strategy Proposal

This document proposes a method for writing robot soccer strategies using a behavior tree framework.

## Basic Concepts

### Behavior Trees

Behaviors are built using a fluent builder API:

```rust
fn create_striker(robot_id: PlayerId) -> impl BehaviorNode {
    BehaviorBuilder::select()
        .when(has_ball(), shoot_at_goal())
        .when(teammate_has_ball(), position_for_pass())
        .do_action(find_ball())
        .build()
}
```

Key components:

- `select()`: Try behaviors in order until one succeeds
- `sequence()`: Execute behaviors in order, all must succeed
- `when(condition, behavior)`: Only run behavior if condition is true
- `do_action()`: Execute a basic action
- `repeat_until()`: Repeat a behavior until a condition is true

### Robot Actions

Basic actions that any robot can perform:

```rust
// Movement
move_to(target: Point2) -> Action
intercept_ball() -> Action
avoid_obstacles() -> Action

// Ball Control
dribble_to(target: Point2) -> Action
kick(power: f32, target: Point2) -> Action
receive_ball() -> Action

// Tactical higher-level actions
mark_opponent(opponent_id: PlayerId) -> Action
block_passing_lane(from: Point2, to: Point2) -> Action
maintain_formation_position() -> Action
```

### Situations

The game situation is a collection of semantic information about the current state of the game.

```rust
pub struct PlayerSituation {
    shot_quality: f32,
    has_ball: bool,
    is_marked: bool,
}
```

Situations can be composed into reusable conditions:

```rust
// Basic situations
let has_ball = Situation::new(|s| s.has_ball, "has_ball");
let ball_nearby = Situation::new(|s| s.ball_distance < 500.0, "ball_nearby");

// Complex situations
let can_shoot = Situation::new(|s| {
    s.has_ball &&
    s.shot_quality > 0.7 &&
    !s.is_marked
}, "can_shoot");

let should_defend = Situation::new(|s| {
    s.ball_in_our_half &&
    s.teammates_behind_ball < 2
}, "should_defend");
```

## Robot Roles

### Striker

```rust
fn create_striker(robot_id: PlayerId) -> impl BehaviorNode {
    BehaviorBuilder::select()
        // Shooting
        .when(can_shoot(),
            BehaviorBuilder::sequence()
                .then(aim_at_goal())
                .then(shoot_at_goal())
                .build())
        // Ball possession
        .when(has_ball(),
            BehaviorBuilder::select()
                .when(teammate_in_better_position(), pass_to_teammate())
                .do_action(dribble_towards_goal()))
        // No ball
        .when(ball_nearby(),
            intercept_ball())
        .do_action(position_for_attack())
        .build()
}
```

### Defender

```rust
fn create_defender(robot_id: PlayerId) -> impl BehaviorNode {
    BehaviorBuilder::select()
        // Clear ball from defense
        .when(dangerous_ball(),
            clear_ball_to_sides())
        // Mark opponents
        .when(should_mark_opponent(),
            BehaviorBuilder::sequence()
                .then(find_most_dangerous_opponent())
                .then(mark_opponent())
                .build())
        // Default positioning
        .do_action(maintain_defensive_position())
        .build()
}
```

### Goalkeeper

```rust
fn create_goalkeeper(robot_id: PlayerId) -> impl BehaviorNode {
    BehaviorBuilder::select()
        // Emergency save
        .when(ball_heading_to_goal(),
            dive_save())
        // Ball in penalty area
        .when(ball_in_penalty_area(),
            clear_ball())
        // Default positioning
        .do_action(position_relative_to_ball())
        .build()
}
```

## Robot Coordination

### Intent Broadcasting

Robots can coordinate loosely by broadcasting their intentions:

```rust
#[derive(Debug, Clone)]
pub enum RobotIntent {
    WillPass { target: Point2 },
    WillShoot { target: Point2 },
    RequestingPass { quality: f32 },
    Defending { zone: Rect },
}

// Example: Broadcasting pass intent
fn pass_to_teammate(robot_id: PlayerId) -> impl BehaviorNode {
    BehaviorBuilder::sequence()
        .then(broadcast_intent(RobotIntent::WillPass {
            target: calculate_pass_target()
        }))
        .then(wait_for_teammate_ready())
        .then(execute_pass())
        .build()
}

// Example: Responding to teammate intent
fn position_for_pass(robot_id: PlayerId) -> impl BehaviorNode {
    BehaviorBuilder::sequence()
        .when(teammate_will_pass(),
            BehaviorBuilder::sequence()
                .then(move_to_pass_target())
                .then(broadcast_intent(RobotIntent::RequestingPass {
                    quality: calculate_position_quality()
                }))
                .then(prepare_to_receive())
                .build())
        .build()
}
```

### Optional: Plays

For more structured coordination, robots can participate in plays:

```rust
fn create_passing_play() -> Play {
    Play {
        name: "Two Robot Pass",
        required_roles: vec![
            PlayRole::Attacker { shoot_threshold: 0.7 },
            PlayRole::Support { min_distance: 2000.0 },
        ],
    }
}
```

## Debugging

Every situation and behavior can include visualizations:

```rust
let scoring_position = Situation::new(|s| s.shot_quality > 0.7, "good_shot")
    .with_visualization(DebugVisualization::Circle {
        radius: 1000.0,
        color: Color::Green,
    });
```

This will show:

- Green circle: Good shooting zone
- Lines: Passing opportunities
- Text: Shot quality metrics

## Best Practices

1. Put emergency responses at highest priority
2. Break complex sequences into named functions
3. Add visualizations for debugging
4. Use composition to build complex behaviors
5. Keep actions simple and focused
6. Use intent broadcasting for loose coordination
7. Only use plays for well-defined set pieces
