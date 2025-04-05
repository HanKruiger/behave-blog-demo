use bevy::prelude::*;
use bevy_behave::prelude::BehaveTree;

use crate::{agent::Agent, grid::GridCell, resizing::GridBounds, schedule::TickSet};

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
    commands.entity(agent).insert(LeftRightWalkNaive::default());
  }
}

fn process_left_right_walk(
  mut q_lr_walkers: Query<(&mut GridCell, &mut LeftRightWalkNaive), With<Agent>>,
  r_grid_bounds: Res<GridBounds>,
) {
  for (mut cell, mut lr_walk) in q_lr_walkers.iter_mut() {
    let target = lr_walk.step_from(&cell);
    if r_grid_bounds.contains(&target) {
      *cell = target;
    } else {
      // if the step would've been out of bounds, reverse and take a step in that direction
      lr_walk.reverse();
      *cell = lr_walk.step_from(&cell);
    }
  }
}

#[derive(Default)]
enum WalkDirectionHorizontal {
  #[default]
  Left,
  Right,
}

#[derive(Component, Default)]
pub struct LeftRightWalkNaive {
  current_direction: WalkDirectionHorizontal,
}

impl LeftRightWalkNaive {
  pub fn step_from(&self, from: &GridCell) -> GridCell {
    match self.current_direction {
      WalkDirectionHorizontal::Left => GridCell::new(from.x - 1, from.y),
      WalkDirectionHorizontal::Right => GridCell::new(from.x + 1, from.y),
    }
  }

  pub fn reverse(&mut self) {
    self.current_direction = match self.current_direction {
      WalkDirectionHorizontal::Left => WalkDirectionHorizontal::Right,
      WalkDirectionHorizontal::Right => WalkDirectionHorizontal::Left,
    }
  }
}

#[derive(Event)]
pub struct SetBehaviourWalkLeftRightNaive;
