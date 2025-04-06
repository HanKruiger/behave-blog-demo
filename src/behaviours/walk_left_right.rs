use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{
  agent::Agent,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

pub struct WalkLeftRightPlugin;

impl Plugin for WalkLeftRightPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_observer(give_left_right_walk_behaviour)
      .add_systems(Update, process_left_right_walk.in_set(TickSet));
  }
}

fn give_left_right_walk_behaviour(
  _trigger: Trigger<SetBehaviourWalkLeftRight>,
  q_agents: Query<Entity, With<Agent>>,
  mut commands: Commands,
) {
  let tree = behave! {
    Behave::Forever => {
      Behave::Sequence => {
        Behave::spawn((
          Name::new("Walk left"),
          WalkInDirectionUntilOutOfBounds((-1, 0)),
        )),
        Behave::spawn((
          Name::new("Walk right"),
          WalkInDirectionUntilOutOfBounds((1, 0)),
        )),
      }
    }
  };

  for agent in q_agents.iter() {
    commands
      .spawn((
        Name::new("Walk left right"),
        BehaveTree::new(tree.clone()).with_logging(true),
      ))
      .set_parent(agent);
  }
}

#[derive(Component, Clone)]
struct WalkInDirectionUntilOutOfBounds(pub (isize, isize));

impl WalkInDirectionUntilOutOfBounds {
  pub fn step_from(&self, from: &GridCell) -> GridCell {
    GridCell::new(from.x + self.0.0, from.y + self.0.1)
  }
}

fn process_left_right_walk(
  b_walk_in_direction: Query<(&WalkInDirectionUntilOutOfBounds, &BehaveCtx)>,
  mut q_agents: Query<&mut GridCell, With<Agent>>,
  r_grid_bounds: Res<GridBounds>,
  mut commands: Commands,
) {
  for (walk, ctx) in b_walk_in_direction.iter() {
    let Ok(mut agent_cell) = q_agents.get_mut(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no GridCell");
      continue;
    };

    let target = walk.step_from(&agent_cell);
    *agent_cell = target;

    let next_target = walk.step_from(&agent_cell);
    if !r_grid_bounds.contains(&next_target) {
      // the next step would've put the agent out of bounds, so we complete the behaviour step
      commands.trigger(ctx.success());
    }
  }
}

#[derive(Event)]
pub struct SetBehaviourWalkLeftRight;
