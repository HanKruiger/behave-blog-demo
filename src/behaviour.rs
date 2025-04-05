use bevy::prelude::*;

use crate::{agent::Agent, grid::GridCell, resizing::GridSize, schedule::TickSet};

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_observer(give_left_right_walk_behaviour)
      .add_systems(Update, process_left_right_walk.in_set(TickSet));
  }
}

fn give_left_right_walk_behaviour(
  _trigger: Trigger<SetBehaviourWalkLeftRightNaive>,
  q_agents: Query<Entity, With<Agent>>,
  mut commands: Commands,
) {
  for agent in q_agents.iter() {
    commands.entity(agent).insert(LeftRightWalk::default());
  }
}

fn process_left_right_walk(
  mut q_left_right_walkers: Query<(&mut GridCell, &mut LeftRightWalk), With<Agent>>,
  r_grid_size: Res<GridSize>,
) {
  for (mut cell, mut lr_walk) in q_left_right_walkers.iter_mut() {
    let target = lr_walk.step_from(&cell);
    if r_grid_size.contains_cell(&target) {
      *cell = target;
    } else {
      lr_walk.reverse();
      *cell = lr_walk.step_from(&cell);
    }
  }
}

#[derive(Default)]
enum WalkDirectionLR {
  #[default]
  Left,
  Right,
}

#[derive(Component, Default)]
pub struct LeftRightWalk {
  current_direction: WalkDirectionLR,
}

impl LeftRightWalk {
  pub fn step_from(&self, from: &GridCell) -> GridCell {
    match self.current_direction {
      WalkDirectionLR::Left => GridCell::new(from.x() - 1, from.y()),
      WalkDirectionLR::Right => GridCell::new(from.x() + 1, from.y()),
    }
  }

  pub fn reverse(&mut self) {
    self.current_direction = match self.current_direction {
      WalkDirectionLR::Left => WalkDirectionLR::Right,
      WalkDirectionLR::Right => WalkDirectionLR::Left,
    }
  }
}

#[derive(Event)]
pub struct SetBehaviourWalkLeftRightNaive;
