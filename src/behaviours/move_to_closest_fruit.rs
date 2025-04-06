use bevy::prelude::*;
use bevy_behave::prelude::*;
use bevy_rand::{global::GlobalEntropy, prelude::WyRand};
use rand::Rng;

use crate::{
  agent::Agent,
  fruit::Fruit,
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

use super::{CurrentMovementBehaviour, MovementBehaviour};

pub fn move_to_closest_fruit_plugin(app: &mut App) {
  app
    .add_systems(Update, process_wander_until_found_fruit.in_set(TickSet))
    .add_systems(Update, process_go_to_nearest_visible_fruit.in_set(TickSet))
    .add_observer(enable_behaviour);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::Sequence => {
        Behave::spawn((
          Name::new("Wander until fruit is found"),
          WanderUntilFoundFruit::new(5),
        )),
        Behave::spawn((
          Name::new("Go to nearest visible fruit"),
          GoToNearestVisibleFruit::new(5),
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

fn process_wander_until_found_fruit(
  b_walk_in_direction: Query<(&WanderUntilFoundFruit, &BehaveCtx)>,
  mut q_agents: Query<&mut GridCell, With<Agent>>,
  q_fruit: Query<&GridCell, (With<Fruit>, Without<Agent>)>,
  r_grid_bounds: Res<GridBounds>,
  mut commands: Commands,
  mut rng: GlobalEntropy<WyRand>,
) {
  for (wander, ctx) in b_walk_in_direction.iter() {
    let Ok(mut agent_cell) = q_agents.get_mut(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no GridCell");
      continue;
    };

    let mut found_fruit = false;

    for fruit_cell in q_fruit.iter() {
      if wander.can_see(&agent_cell, fruit_cell) {
        commands.trigger(ctx.success());
        found_fruit = true;
        break;
      }
    }

    if !found_fruit {
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

fn process_go_to_nearest_visible_fruit(
  b_go_to_nearest: Query<(&GoToNearestVisibleFruit, &BehaveCtx)>,
  mut q_agents: Query<&mut GridCell, With<Agent>>,
  q_fruit: Query<&GridCell, (With<Fruit>, Without<Agent>)>,
  mut commands: Commands,
) {
  for (go_to_nearest, ctx) in b_go_to_nearest.iter() {
    let Ok(mut agent_cell) = q_agents.get_mut(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no GridCell");
      continue;
    };

    let fruit_cell = q_fruit
      .iter()
      .filter(|f| go_to_nearest.can_see(&agent_cell, f))
      .min_by(|f1, f2| {
        f1.distance(&agent_cell)
          .total_cmp(&f2.distance(&agent_cell))
      });

    let Some(fruit_cell) = fruit_cell else {
      commands.trigger(ctx.failure());

      continue;
    };

    if *fruit_cell == *agent_cell {
      // we're sitting on the fruit, we have made it!
      commands.trigger(ctx.success());
      continue;
    }

    agent_cell.step_to(fruit_cell);
  }
}

#[derive(Component, Clone)]
struct WanderUntilFoundFruit {
  viewing_distance: usize,
}

impl WanderUntilFoundFruit {
  pub fn new(viewing_distance: usize) -> Self {
    Self { viewing_distance }
  }

  pub fn can_see(&self, from: &GridCell, to: &GridCell) -> bool {
    from.distance(to) <= self.viewing_distance as f32
  }
}

#[derive(Component, Clone)]
struct GoToNearestVisibleFruit {
  viewing_distance: usize,
}

impl GoToNearestVisibleFruit {
  pub fn new(viewing_distance: usize) -> Self {
    Self { viewing_distance }
  }

  pub fn can_see(&self, from: &GridCell, to: &GridCell) -> bool {
    from.distance(to) <= self.viewing_distance as f32
  }
}

#[derive(Event)]
pub struct SetBehaviourMoveToClosestFruit;
