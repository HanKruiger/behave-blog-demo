use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;
use bevy_behave::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

pub fn fruit_plugin(app: &mut App) {
  app
    .add_systems(Update, process_spawn_fruit_task.in_set(TickSet))
    .add_observer(spawn_fruit_spawner);
}

fn spawn_fruit_spawner(
  _trigger: Trigger<SpawnFruitSpawner>,
  fruit_spawner: Option<Single<&FruitSpawner>>,
  mut commands: Commands,
) {
  if fruit_spawner.is_some() {
    // we do not want >1 fruit spawner
    return;
  }

  let mut fruit_spawner = commands.spawn((FruitSpawner {
    target_fruit_number: 20, // maybe do based on grid size?
  },));

  let tree = behave!(
    Behave::Forever => {
      Behave::spawn((
        Name::new("Spawn fruit until enough"),
        SpawnFruitUntilEnough,
      ))
    }
  );

  fruit_spawner.with_child((
    Name::new("Spawn fruits"),
    BehaveTree::new(tree.clone()).with_logging(true),
  ));
}

fn process_spawn_fruit_task(
  b_spawn_until_enough: Query<&BehaveCtx, With<SpawnFruitUntilEnough>>,
  q_fruit_spawners: Query<&FruitSpawner>,
  q_fruit: Query<(), With<Fruit>>,
  r_grid_bounds: Res<GridBounds>,
  mut r_meshes: ResMut<Assets<Mesh>>,
  mut r_materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
  mut rng: GlobalEntropy<WyRand>,
) {
  let mut n_fruit = None;
  for ctx in b_spawn_until_enough.iter() {
    let Ok(spawner) = q_fruit_spawners.get(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no FruitSpawner");
      continue;
    };

    if n_fruit.is_none() {
      n_fruit = Some(q_fruit.iter().count());
    }

    if n_fruit.unwrap() < spawner.target_fruit_number {
      let cell = r_grid_bounds.get_random_position(&mut rng);
      commands.spawn((
        Fruit::new(2),
        cell,
        Mesh2d(r_meshes.add(Rectangle::new(0.3, 0.3))),
        MeshMaterial2d(r_materials.add(Color::from(tw::RED_600))),
      ));
    }
  }
}

#[derive(Component)]
struct FruitSpawner {
  target_fruit_number: usize,
}

#[derive(Component, Clone)]
struct SpawnFruitUntilEnough;

#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0.0, 0.0, 0.09)), GridCell)]
pub struct Fruit {
  pub nutritional_value: usize,
}

impl Fruit {
  pub fn new(nutritional_value: usize) -> Self {
    Self { nutritional_value }
  }
}

#[derive(Event)]
pub struct SpawnFruitSpawner;
