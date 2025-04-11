use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;
use bevy_behave::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};

use crate::{
  grid::{GridBounds, GridCell},
  schedule::TickSet,
};

pub fn coins_plugin(app: &mut App) {
  app
    .add_systems(Update, process_spawn_coins_task.in_set(TickSet))
    .add_observer(spawn_coins_spawner);
}

fn spawn_coins_spawner(
  _trigger: Trigger<SpawnCoinSpawner>,
  coins_spawner: Option<Single<&CoinSpawner>>,
  mut commands: Commands,
) {
  if coins_spawner.is_some() {
    // we do not want >1 coin spawner
    return;
  }

  let mut coins_spawner = commands.spawn((CoinSpawner {
    target_coin_number: 10, // maybe do based on grid size?
  },));

  let tree = behave!(
    Behave::Forever => {
      Behave::spawn((
        Name::new("Spawn coins until enough"),
        SpawnCoinsUntilEnough,
      ))
    }
  );

  coins_spawner.with_child((
    Name::new("Spawn coins"),
    BehaveTree::new(tree.clone()).with_logging(false),
  ));
}

fn process_spawn_coins_task(
  b_spawn_until_enough: Query<&BehaveCtx, With<SpawnCoinsUntilEnough>>,
  q_coin_spawners: Query<&CoinSpawner>,
  q_coins: Query<(), With<Coin>>,
  r_grid_bounds: Res<GridBounds>,
  mut r_meshes: ResMut<Assets<Mesh>>,
  mut r_materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
  mut rng: GlobalEntropy<WyRand>,
) {
  let mut n_coins = None;
  for ctx in b_spawn_until_enough.iter() {
    let Ok(spawner) = q_coin_spawners.get(ctx.target_entity()) else {
      warn!("skipping behaviour that points to entity with no CoinSpawner");
      continue;
    };

    if n_coins.is_none() {
      n_coins = Some(q_coins.iter().count());
    }

    if n_coins.unwrap() < spawner.target_coin_number {
      let cell = r_grid_bounds.get_random_position(&mut rng);
      commands.spawn((
        Coin::new(2),
        cell,
        Mesh2d(r_meshes.add(Circle::new(0.25))),
        MeshMaterial2d(r_materials.add(Color::from(tw::YELLOW_400))),
      ));
    }
  }
}

#[derive(Component)]
struct CoinSpawner {
  target_coin_number: usize,
}

#[derive(Component, Clone)]
struct SpawnCoinsUntilEnough;

#[derive(Component)]
#[require(Transform(|| Transform::from_xyz(0.0, 0.0, 0.08)), GridCell)]
pub struct Coin {
  pub monetary_value: usize,
}

impl Coin {
  pub fn new(monetary_value: usize) -> Self {
    Self { monetary_value }
  }
}

#[derive(Event)]
pub struct SpawnCoinSpawner;
