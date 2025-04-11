use bevy::prelude::*;
use bevy_behave::prelude::*;
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};
use rand::Rng;

use crate::{
  agent::Agent,
  coins::Coin,
  fruit::Fruit,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

pub fn target_finding_plugin(app: &mut App) {
  app.add_systems(
    Update,
    (process_find_target, process_go_to_target).in_set(TickSet),
  );
}

fn process_find_target(
  b_find_target: Query<(&FindTarget, &BehaveCtx)>,
  mut q_agents: Query<(Entity, &mut GridCell), With<Agent>>,
  q_fruits: Query<(Entity, &GridCell), (With<Fruit>, Without<Agent>)>,
  q_coins: Query<(Entity, &GridCell), (With<Coin>, Without<Agent>)>,
  r_grid_bounds: Res<GridBounds>,
  mut commands: Commands,
  mut rng: GlobalEntropy<WyRand>,
) {
  for (find_target, ctx) in b_find_target.iter() {
    let Ok((agent, mut agent_cell)) = q_agents.get_mut(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no GridCell");
      continue;
    };

    let mut closest = None;
    let mut closest_dist = f32::MAX;

    let options: Box<dyn Iterator<Item = (Entity, &GridCell)>> = match find_target.kind {
      TargetKind::Fruit => Box::new(q_fruits.iter()),
      TargetKind::Coins => Box::new(q_coins.iter()),
    };

    for (e, cell) in options {
      if find_target.can_see(&agent_cell, cell) {
        if closest.is_some() {
          let dist = agent_cell.distance(cell);
          if dist < closest_dist {
            closest_dist = dist;
            closest = Some(e);
          }
        } else {
          closest = Some(e)
        }
      }
    }

    // insert a reference to the target as a component in the agent entity
    if let Some(e) = closest {
      commands.entity(agent).insert(Target(e));
      commands.trigger(ctx.success());
    } else {
      // wander randomly

      let options = agent_cell
        .neighbours()
        .into_iter()
        .filter(|c| r_grid_bounds.contains(c))
        .collect::<Vec<GridCell>>();

      let index = rng.gen_range(0..options.len());
      let target = options[index];

      *agent_cell = target;
    }
  }
}

fn process_go_to_target(
  b_go_to_nearest: Query<&BehaveCtx, With<GoToTarget>>,
  mut q_agents: Query<(Entity, &mut GridCell, &Target), With<Agent>>,
  q_targets: Query<&GridCell, Without<Agent>>,
  mut commands: Commands,
) {
  for ctx in b_go_to_nearest.iter() {
    let Ok((agent, mut agent_cell, target_fruit)) = q_agents.get_mut(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no GridCell");
      continue;
    };

    let fruit_cell = q_targets.get(target_fruit.0);

    let Ok(fruit_cell) = fruit_cell else {
      // fruit must've disappeared (or eaten by us)
      commands.entity(agent).remove::<Target>();
      commands.trigger(ctx.success());
      continue;
    };

    if *fruit_cell == *agent_cell {
      // we're sitting on the fruit, we have made it!
      commands.entity(agent).remove::<Target>();
      commands.trigger(ctx.success());
    } else {
      // we're not quite there yet, take a step in the right direction
      agent_cell.step_to(fruit_cell);
    }
  }
}

#[derive(Component, Clone)]
pub struct FindTarget {
  kind: TargetKind,
  viewing_distance: usize,
}

impl FindTarget {
  pub fn new(kind: TargetKind, viewing_distance: usize) -> Self {
    Self {
      kind,
      viewing_distance,
    }
  }

  pub fn can_see(&self, from: &GridCell, to: &GridCell) -> bool {
    from.distance(to) <= self.viewing_distance as f32
  }
}

#[derive(Clone)]
pub enum TargetKind {
  Fruit,
  Coins,
}

#[derive(Component)]
struct Target(pub Entity);

#[derive(Component, Clone)]
pub struct GoToTarget;
