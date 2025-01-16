use super::situation::RobotSituation;
use crate::types::PlayerId;
use std::marker::PhantomData;

mod actions;

pub use actions::*;

pub enum NodeResult {
    Success,
    Failure(String),
    Running,
}

pub trait BehaviorNode {
    fn tick(&mut self, ctx: &mut BehaviorContext) -> NodeResult;
}

pub struct BehaviorContext {
    pub robot_id: PlayerId,
    pub situation: RobotSituation,
}

pub struct BehaviorBuilder<N: BehaviorNode> {
    node: N,
    _phantom: PhantomData<N>,
}

pub struct SelectNode {
    pub children: Vec<Box<dyn BehaviorNode>>,
}

pub struct SequenceNode {
    pub children: Vec<Box<dyn BehaviorNode>>,
    pub current_child: usize,
}

impl BehaviorNode for SelectNode {
    fn tick(&mut self, ctx: &mut BehaviorContext) -> NodeResult {
        for child in &mut self.children {
            match child.tick(ctx) {
                NodeResult::Success => return NodeResult::Success,
                NodeResult::Running => return NodeResult::Running,
                NodeResult::Failure(_) => continue,
            }
        }
        NodeResult::Failure("All children failed".into())
    }
}

impl BehaviorNode for SequenceNode {
    fn tick(&mut self, ctx: &mut BehaviorContext) -> NodeResult {
        while self.current_child < self.children.len() {
            match self.children[self.current_child].tick(ctx) {
                NodeResult::Success => {
                    self.current_child += 1;
                    continue;
                }
                NodeResult::Running => return NodeResult::Running,
                NodeResult::Failure(reason) => {
                    self.current_child = 0;
                    return NodeResult::Failure(reason);
                }
            }
        }
        self.current_child = 0;
        NodeResult::Success
    }
}

// Helper function to create behavior trees
pub fn select() -> SelectNode {
    SelectNode {
        children: Vec::new(),
    }
}

pub fn sequence() -> SequenceNode {
    SequenceNode {
        children: Vec::new(),
        current_child: 0,
    }
}
