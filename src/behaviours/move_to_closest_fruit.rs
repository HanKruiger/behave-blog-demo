use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{
  agent::Agent,
  behaviours::target_finding::{FindTarget, GoToTarget, TargetKind},
};

use super::{CurrentMovementBehaviour, MovementBehaviour};

pub fn move_to_closest_fruit_plugin(app: &mut App) {
  app.add_observer(enable_behaviour);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::Sequence => {
        Behave::spawn((
          Name::new("Find fruit"),
          FindTarget::new(TargetKind::Fruit, 8),
        )),
        Behave::spawn((
          Name::new("Go to target fruit"),
          GoToTarget,
        )),
      }
    }
  }
}

fn enable_behaviour(
  _trigger: Trigger<SetBehaviourMoveToClosestFruit>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_current_movement_behaviour: ResMut<CurrentMovementBehaviour>,
  mut commands: Commands,
) {
  let tree = build_behaviour_tree();
  let name = "Move to closest fruit";

  r_current_movement_behaviour.0 = Some((tree.clone(), name.into()));

  for agent in q_agents.iter() {
    commands
      .spawn((
        Name::new(name),
        BehaveTree::new(tree.clone()).with_logging(true),
        MovementBehaviour,
      ))
      .set_parent(agent);
  }
}

#[derive(Event)]
pub struct SetBehaviourMoveToClosestFruit;
