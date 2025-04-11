use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{
  agent::Agent,
  behaviours::target_finding::{FindTarget, GoToTarget, TargetKind},
  hunger::Hunger,
};

use super::{CurrentMovementBehaviour, MovementBehaviour};

pub fn hunger_based_plugin(app: &mut App) {
  app
    .add_observer(enable_behaviour)
    .add_observer(on_hunger_check);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::Sequence => {
        Behave::IfThen => {
          Behave::trigger(HungerCheck(0.4)),

          // spawned if hunger check succeeded
          Behave::spawn((
            Name::new("Find fruit"),
            FindTarget::new(TargetKind::Fruit, 8),
          )),

          // spawned if hunger check failed
          Behave::spawn((
            Name::new("Find coins"),
            FindTarget::new(TargetKind::Coins, 8),
          )),
        },

        // go to the target we just found
        Behave::spawn((
          Name::new("Go to target"),
          GoToTarget,
        )),
      }
    }
  }
}

fn enable_behaviour(
  _trigger: Trigger<SetBehaviourHungerBased>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_current_movement_behaviour: ResMut<CurrentMovementBehaviour>,
  mut commands: Commands,
) {
  let tree = build_behaviour_tree();
  let name = "Hunger based movement";

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

fn on_hunger_check(
  trigger: Trigger<BehaveTrigger<HungerCheck>>,
  q_agents: Query<&Hunger, With<Agent>>,
  mut commands: Commands,
) {
  let ctx = trigger.event().ctx();
  let hunger_check = trigger.event().inner();
  let Ok(hunger) = q_agents.get(ctx.target_entity()) else {
    return;
  };
  if hunger.fraction_left() < hunger_check.0 {
    // agent is hungry -> report success for the check!
    commands.trigger(ctx.success());
  } else {
    // agent is not hungry -> report failure for the check!
    commands.trigger(ctx.failure());
  }
}

#[derive(Event, Clone)]
struct HungerCheck(pub f32);

#[derive(Event)]
pub struct SetBehaviourHungerBased;
