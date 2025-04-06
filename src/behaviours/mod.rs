mod eating;
mod move_to_closest_fruit;
mod walk_left_right;
mod walk_left_right_naive;

use bevy::prelude::*;
use bevy_behave::prelude::*;

use walk_left_right_naive::WalkInDirectionUntilOutOfBounds as WalkInDirectionUntilOutOfBoundsNaive;

pub use eating::EnableEating;
pub use move_to_closest_fruit::SetBehaviourMoveToClosestFruit;
pub use walk_left_right::SetBehaviourWalkLeftRight;
pub use walk_left_right_naive::SetBehaviourWalkLeftRightNaive;

use crate::agent::Agent;

pub fn behaviours_plugin(app: &mut App) {
  app
    .init_resource::<CurrentMovementBehaviour>()
    .init_resource::<NaiveMovementEnabled>()
    .add_plugins(BehavePlugin::default())
    .add_plugins((
      walk_left_right_naive::walk_left_right_naive_plugin,
      walk_left_right::walk_left_right_plugin,
      move_to_closest_fruit::move_to_closest_fruit_plugin,
      eating::eating_plugin,
    ))
    .add_systems(Update, on_agent_spawn_insert_movement_behaviour)
    .add_observer(on_clear_naive_movement_behaviours)
    .add_observer(on_clear_movement_behaviours);
}

/// Clears all Bevy Behave movement behaviours for existing and new agents
fn on_clear_movement_behaviours(
  _trigger: Trigger<DisableMovementBehaviours>,
  q_agents: Query<Entity, With<Agent>>,
  q_movement_behaviours: Query<(Entity, &Parent), (With<BehaveTree>, With<MovementBehaviour>)>,
  mut r_current_movement_behaviour: ResMut<CurrentMovementBehaviour>,
  mut commands: Commands,
) {
  r_current_movement_behaviour.0 = None;
  for (tree, parent) in q_movement_behaviours.iter() {
    if q_agents.contains(parent.get()) {
      commands.entity(tree).despawn_recursive();
    }
  }
}

/// Clears all naive movement behaviours for existing and new
fn on_clear_naive_movement_behaviours(
  _trigger: Trigger<DisableNaiveMovementBehaviours>,
  w_walk_naive: Query<Entity, With<WalkInDirectionUntilOutOfBoundsNaive>>,
  mut r_naive_movement_enabled: ResMut<NaiveMovementEnabled>,
  mut commands: Commands,
) {
  r_naive_movement_enabled.0 = false;
  for e in w_walk_naive.iter() {
    commands
      .entity(e)
      .remove::<WalkInDirectionUntilOutOfBoundsNaive>();
  }
}

fn on_agent_spawn_insert_movement_behaviour(
  q_new_agents: Query<Entity, Added<Agent>>,
  r_current_movement_behaviour: Res<CurrentMovementBehaviour>,
  r_naive_movement_enabled: Res<NaiveMovementEnabled>,
  mut commands: Commands,
) {
  if r_naive_movement_enabled.0 {
    for agent in q_new_agents.iter() {
      commands
        .entity(agent)
        .insert(WalkInDirectionUntilOutOfBoundsNaive::new(-1, 0));
    }
  } else if let Some((tree, name)) = &r_current_movement_behaviour.0 {
    for agent in q_new_agents.iter() {
      commands
        .spawn((
          Name::new(name.clone()),
          BehaveTree::new(tree.clone()).with_logging(true),
          MovementBehaviour,
        ))
        .set_parent(agent);
    }
  }
}

#[derive(Event)]
pub struct DisableMovementBehaviours;

#[derive(Event)]
pub struct DisableNaiveMovementBehaviours;

#[derive(Resource, Default)]
struct NaiveMovementEnabled(pub bool);

#[derive(Component)]
struct MovementBehaviour;

#[derive(Resource, Default)]
struct CurrentMovementBehaviour(pub Option<(Tree<Behave>, String)>);
