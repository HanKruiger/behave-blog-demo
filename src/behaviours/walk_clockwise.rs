use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{agent::Agent, behaviours::walking::WalkInDirectionUntilOutOfBounds};

use super::{CurrentMovementBehaviour, MovementBehaviour};

pub fn walk_clockwise_plugin(app: &mut App) {
  app.add_observer(enable_behaviour);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::Sequence => {
        Behave::spawn((
          Name::new("Walk left"),
          WalkInDirectionUntilOutOfBounds((-1, 0)),
        )),
        Behave::spawn((
          Name::new("Walk up"),
          WalkInDirectionUntilOutOfBounds((0, 1)),
        )),
        Behave::spawn((
          Name::new("Walk right"),
          WalkInDirectionUntilOutOfBounds((1, 0)),
        )),
        Behave::spawn((
          Name::new("Walk down"),
          WalkInDirectionUntilOutOfBounds((0, -1)),
        )),
      }
    }
  }
}

fn enable_behaviour(
  _trigger: Trigger<SetBehaviourWalkClockwise>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_current_movement_behaviour: ResMut<CurrentMovementBehaviour>,
  mut commands: Commands,
) {
  let tree = build_behaviour_tree();
  let name = "Walk clockwise";

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
pub struct SetBehaviourWalkClockwise;
