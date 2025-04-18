use crate::{
  agent::Agent,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};
use bevy::prelude::*;
use bevy_behave::prelude::*;

pub fn walking_plugin(app: &mut App) {
  app.add_systems(Update, process_walk_in_direction.in_set(TickSet));
}

#[derive(Component, Clone)]
pub struct WalkInDirectionUntilOutOfBounds(pub (isize, isize));

impl WalkInDirectionUntilOutOfBounds {
  pub fn new(x: isize, y: isize) -> Self {
    Self((x, y))
  }

  pub fn reverse(&mut self) {
    self.0.0 = -self.0.0;
    self.0.1 = -self.0.1;
  }

  pub fn step_from(&self, from: &GridCell) -> GridCell {
    GridCell::new(from.x + self.0.0, from.y + self.0.1)
  }
}

fn process_walk_in_direction(
  q_walks: Query<(&WalkInDirectionUntilOutOfBounds, &BehaveCtx)>,
  mut q_agent_cells: Query<&mut GridCell, With<Agent>>,
  r_bounds: Res<GridBounds>,
  mut commands: Commands,
) {
  for (walk, ctx) in q_walks.iter() {
    let Ok(mut agent_cell) = q_agent_cells.get_mut(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no GridCell");
      continue;
    };

    let target = walk.step_from(&agent_cell);
    *agent_cell = target;

    let next_target = walk.step_from(&agent_cell);
    if !r_bounds.contains(&next_target) {
      // the next step would've put the agent out of bounds, so we successfully completed the behaviour step
      commands.trigger(ctx.success());
    }
  }
}
