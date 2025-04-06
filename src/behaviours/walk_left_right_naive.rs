use bevy::prelude::*;
use bevy_behave::prelude::BehaveTree;

use crate::{
  agent::Agent,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

pub struct WalkLeftRightNaivePlugin;

impl Plugin for WalkLeftRightNaivePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_observer(set_behaviour)
      .add_systems(Update, process_left_right_walk.in_set(TickSet));
  }
}

fn set_behaviour(
  _trigger: Trigger<SetBehaviourWalkLeftRightNaive>,
  q_agents: Query<Entity, With<Agent>>,
  q_behaviours: Query<(Entity, &Parent), With<BehaveTree>>,
  mut commands: Commands,
) {
  // first, clear any bevy_behave behaviour trees on agents
  for (tree, parent) in q_behaviours.iter() {
    if q_agents.contains(parent.get()) {
      commands.entity(tree).despawn_recursive();
    }
  }

  for agent in q_agents.iter() {
    commands
      .entity(agent)
      .insert(WalkInDirectionUntilOutOfBounds::new(-1, 0));
  }
}

fn process_left_right_walk(
  mut q_walkers: Query<(&mut GridCell, &mut WalkInDirectionUntilOutOfBounds), With<Agent>>,
  r_grid_bounds: Res<GridBounds>,
) {
  for (mut cell, mut walk) in q_walkers.iter_mut() {
    let target = walk.step_from(&cell);
    *cell = target;

    let next_target = walk.step_from(&cell);
    if !r_grid_bounds.contains(&next_target) {
      // the next step would've put the agent out of bounds, so we reverse
      walk.reverse();
    }
  }
}

#[derive(Component, Clone)]
pub struct WalkInDirectionUntilOutOfBounds {
  x: isize,
  y: isize,
}

impl WalkInDirectionUntilOutOfBounds {
  pub fn new(x: isize, y: isize) -> Self {
    Self { x, y }
  }

  pub fn step_from(&self, from: &GridCell) -> GridCell {
    GridCell::new(from.x + self.x, from.y + self.y)
  }

  pub fn reverse(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
  }
}

#[derive(Event)]
pub struct SetBehaviourWalkLeftRightNaive;
