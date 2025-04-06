use bevy::prelude::*;
use bevy_behave::prelude::*;

use crate::{agent::Agent, fruit::Fruit, grid::GridCell, hunger::Hunger};

pub fn eating_plugin(app: &mut App) {
  app
    .init_resource::<EatingEnabled>()
    .add_systems(Update, process_eat_fruit)
    .add_systems(Update, on_agent_spawn_insert_eating_behaviour)
    .add_observer(enable_behaviour);
}

fn build_behaviour_tree() -> Tree<bevy_behave::Behave> {
  behave! {
    Behave::Forever => {
      Behave::spawn((
        Name::new("Eat fruit when there's fruit"),
        EatFruit,
      )),
    }
  }
}

fn enable_behaviour(
  _trigger: Trigger<EnableEating>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_eating_enabled: ResMut<EatingEnabled>,
  mut commands: Commands,
) {
  r_eating_enabled.0 = true;

  let name = "Eat fruit when on fruit";
  let tree = build_behaviour_tree();

  for agent in q_agents.iter() {
    commands
      .spawn((
        Name::new(name),
        BehaveTree::new(tree.clone()).with_logging(true),
        EatingBehaviour,
      ))
      .set_parent(agent);
  }
}

fn process_eat_fruit(
  b_eat_fruit: Query<&BehaveCtx, (With<EatFruit>, Without<Agent>)>,
  mut q_hunger: Query<(&mut Hunger, &GridCell), With<Agent>>,
  q_fruit: Query<(Entity, &Fruit, &GridCell), Without<Agent>>,
  mut commands: Commands,
) {
  for ctx in b_eat_fruit.iter() {
    let Ok((mut agent_hunger, agent_cell)) = q_hunger.get_mut(ctx.target_entity()) else {
      continue;
    };
    for (fruit_entity, fruit, fruit_cell) in q_fruit.iter() {
      if fruit_cell == agent_cell {
        commands.entity(fruit_entity).despawn_recursive();
        agent_hunger.eat(fruit.nutritional_value);
      }
    }
  }
}

fn on_agent_spawn_insert_eating_behaviour(
  q_new_agents: Query<Entity, Added<Agent>>,
  r_eating_enabled: Res<EatingEnabled>,
  mut commands: Commands,
) {
  if r_eating_enabled.0 {
    let name = "Eat fruit when on fruit";
    let tree = build_behaviour_tree();

    for agent in q_new_agents.iter() {
      commands
        .spawn((
          Name::new(name),
          BehaveTree::new(tree.clone()).with_logging(true),
          EatingBehaviour,
        ))
        .set_parent(agent);
    }
  }
}

#[derive(Component, Clone)]
struct EatFruit;

#[derive(Component)]
struct EatingBehaviour;

#[derive(Event)]
pub struct EnableEating;

#[derive(Resource, Default)]
pub struct EatingEnabled(pub bool);
