use bevy::prelude::*;

use crate::{
  agent::Agent,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

use super::{NaiveMovementEnabled, walking::WalkInDirectionUntilOutOfBounds};

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
  r_bounds: Res<GridBounds>,
) {
  // loop over all grid cells & walk components that
  // are attached to agents
  for (mut grid_cell, mut walk) in q_walkers.iter_mut() {
    // determine the next step, and update the agent's
    // grid cell (make it move there)
    *grid_cell = walk.step_from(&grid_cell);

    // let's see if the next step will put us out of bounds
    let next_target = walk.step_from(&grid_cell);
    if !r_bounds.contains(&next_target) {
      // the next step would've put the agent out of bounds,
      // so we reverse (basically just flip -1 to +1 and
      // vice versa)
      walk.reverse();
    }
  }
}

#[derive(Event)]
pub struct SetBehaviourWalkLeftRightNaive;
