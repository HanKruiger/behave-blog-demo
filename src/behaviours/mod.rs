mod hunger_based;
mod move_to_closest_fruit;
mod pickups;
mod target_finding;
mod walk_clockwise;
mod walk_left_right;
mod walk_left_right_naive;
mod walking;

use bevy::prelude::*;
use bevy_behave::prelude::*;

pub use hunger_based::SetBehaviourHungerBased;
pub use move_to_closest_fruit::SetBehaviourMoveToClosestFruit;
pub use walk_clockwise::SetBehaviourWalkClockwise;
pub use walk_left_right::SetBehaviourWalkLeftRight;
pub use walk_left_right_naive::SetBehaviourWalkLeftRightNaive;
use walking::WalkInDirectionUntilOutOfBounds;

use crate::agent::Agent;

pub fn behaviours_plugin(app: &mut App) {
  app
    .init_resource::<CurrentMovementBehaviour>()
    .init_resource::<NaiveMovementEnabled>()
    .add_plugins(BehavePlugin::default())
    .add_plugins((
      walk_left_right_naive::walk_left_right_naive_plugin,
      walking::walking_plugin,
      walk_left_right::walk_left_right_plugin,
      walk_clockwise::walk_clockwise_plugin,
      move_to_closest_fruit::move_to_closest_fruit_plugin,
      hunger_based::hunger_based_plugin,
      target_finding::target_finding_plugin,
      pickups::pickups_plugin,
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

/// Clears all naive movement behaviours for existing and new agents
fn on_clear_naive_movement_behaviours(
  _trigger: Trigger<DisableNaiveMovementBehaviours>,
  w_walk_naive: Query<Entity, (With<WalkInDirectionUntilOutOfBounds>, With<Agent>)>,
  mut r_naive_movement_enabled: ResMut<NaiveMovementEnabled>,
  mut commands: Commands,
) {
  r_naive_movement_enabled.0 = false;
  for e in w_walk_naive.iter() {
    commands
      .entity(e)
      .remove::<WalkInDirectionUntilOutOfBounds>();
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
        .insert(WalkInDirectionUntilOutOfBounds((-1, 0)));
    }
  } else if let Some((tree, name)) = &r_current_movement_behaviour.0 {
    for agent in q_new_agents.iter() {
      commands
        .spawn((
          Name::new(name.clone()),
          BehaveTree::new(tree.clone()).with_logging(false),
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
