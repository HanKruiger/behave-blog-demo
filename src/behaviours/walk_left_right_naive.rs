use bevy::prelude::*;

use crate::{
  agent::Agent,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

use super::{walking::WalkInDirectionUntilOutOfBounds, NaiveMovementEnabled};

pub fn walk_left_right_naive_plugin(app: &mut App) {
  app
    .add_observer(enable_behaviour)
    .add_systems(Update, process_left_right_walk.in_set(TickSet));
}

fn enable_behaviour(
  _trigger: Trigger<SetBehaviourWalkLeftRightNaive>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_naive_movement_enabled: ResMut<NaiveMovementEnabled>,
  mut commands: Commands,
) {
  r_naive_movement_enabled.0 = true;

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

#[derive(Event)]
pub struct SetBehaviourWalkLeftRightNaive;
