mod walk_left_right;
mod walk_left_right_naive;

use bevy::prelude::*;

use bevy_behave::prelude::{BehavePlugin, BehaveTree};
use walk_left_right_naive::WalkInDirectionUntilOutOfBounds as WalkInDirectionUntilOutOfBoundsNaive;

pub use walk_left_right::SetBehaviourWalkLeftRight;
pub use walk_left_right_naive::SetBehaviourWalkLeftRightNaive;

use crate::agent::Agent;

pub struct BehavioursPlugin;

impl Plugin for BehavioursPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(BehavePlugin::default())
      .add_plugins((
        walk_left_right_naive::WalkLeftRightNaivePlugin,
        walk_left_right::WalkLeftRightPlugin,
      ))
      .add_observer(on_clear_naive_behaviours)
      .add_observer(on_clear_behaviours);
  }
}

/// Clears all behaviours
fn on_clear_behaviours(
  _trigger: Trigger<ClearBehaviours>,
  q_agents: Query<Entity, With<Agent>>,
  q_behaviours: Query<(Entity, &Parent), With<BehaveTree>>,
  mut commands: Commands,
) {
  for (tree, parent) in q_behaviours.iter() {
    if q_agents.contains(parent.get()) {
      commands.entity(tree).despawn_recursive();
    }
  }
}

/// Clears all naive behaviours
fn on_clear_naive_behaviours(
  _trigger: Trigger<ClearNaiveBehaviours>,
  w_walk_naive: Query<Entity, With<WalkInDirectionUntilOutOfBoundsNaive>>,
  mut commands: Commands,
) {
  for e in w_walk_naive.iter() {
    commands
      .entity(e)
      .remove::<WalkInDirectionUntilOutOfBoundsNaive>();
  }
}

#[derive(Event)]
pub struct ClearBehaviours;
#[derive(Event)]
pub struct ClearNaiveBehaviours;
