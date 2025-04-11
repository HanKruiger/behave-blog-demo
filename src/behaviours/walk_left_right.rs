use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{agent::Agent, behaviours::walking::WalkInDirectionUntilOutOfBounds};

use super::{CurrentMovementBehaviour, MovementBehaviour};

pub fn walk_left_right_plugin(app: &mut App) {
  app.add_observer(enable_behaviour);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::Sequence => {
        Behave::spawn((
          Name::new("Walk left"),
          WalkInDirectionUntilOutOfBounds::new(-1, 0),
        )),
        Behave::spawn((
          Name::new("Walk right"),
          WalkInDirectionUntilOutOfBounds::new(1, 0),
        )),
      }
    }
  }
}

fn enable_behaviour(
  _trigger: Trigger<SetBehaviourWalkLeftRight>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_current_movement_behaviour: ResMut<CurrentMovementBehaviour>,
  mut commands: Commands,
) {
  let tree = build_behaviour_tree();
  let name = "Walk left right";

  r_current_movement_behaviour.0 = Some((tree.clone(), name.into()));

  for agent in q_agents.iter() {
    commands
      .spawn((
        Name::new(name),
        BehaveTree::new(tree.clone()).with_logging(false),
        MovementBehaviour,
      ))
      .set_parent(agent);
  }
}

#[derive(Event)]
pub struct SetBehaviourWalkLeftRight;
