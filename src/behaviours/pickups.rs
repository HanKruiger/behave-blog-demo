use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{
  agent::Agent, coins::Coin, fruit::Fruit, grid::GridCell, hunger::Hunger, points::Points,
};

pub fn pickups_plugin(app: &mut App) {
  app
    .add_systems(Update, process_pick_ups)
    .add_systems(Update, on_agent_spawn_insert_pick_up_behaviour);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::spawn((
        Name::new("Eat fruit when there's fruit"),
        PickUpStuff,
      )),
    }
  }
}

fn process_pick_ups(
  b_pick_up_stuff: Query<&BehaveCtx, (With<PickUpStuff>, Without<Agent>)>,
  mut q_hunger: Query<(Option<&mut Hunger>, &mut Points, &GridCell), With<Agent>>,
  q_fruit: Query<(Entity, &Fruit, &GridCell), Without<Agent>>,
  q_coins: Query<(Entity, &Coin, &GridCell), Without<Agent>>,
  mut commands: Commands,
) {
  for ctx in b_pick_up_stuff.iter() {
    let Ok((agent_hunger, mut agent_points, agent_cell)) = q_hunger.get_mut(ctx.target_entity())
    else {
      continue;
    };
    if let Some(mut agent_hunger) = agent_hunger {
      for (fruit_entity, fruit, fruit_cell) in q_fruit.iter() {
        if fruit_cell == agent_cell {
          commands.entity(fruit_entity).despawn_recursive();
          agent_hunger.eat(fruit.nutritional_value);
        }
      }
    }
    for (coin_entity, coin, coin_cell) in q_coins.iter() {
      if coin_cell == agent_cell {
        commands.entity(coin_entity).despawn_recursive();
        agent_points.earn(coin.monetary_value);
      }
    }
  }
}

fn on_agent_spawn_insert_pick_up_behaviour(
  q_new_agents: Query<Entity, Added<Agent>>,
  mut commands: Commands,
) {
  let name = "Pick up items when on an item";
  let tree = build_behaviour_tree();

  for agent in q_new_agents.iter() {
    commands
      .spawn((
        Name::new(name),
        BehaveTree::new(tree.clone()).with_logging(false),
        PickUpBehaviour,
      ))
      .set_parent(agent);
  }
}

#[derive(Component, Clone)]
struct PickUpStuff;

#[derive(Component)]
struct PickUpBehaviour;
