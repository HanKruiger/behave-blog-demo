use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;

use crate::agent::Agent;
use crate::schedule::HungerTickSet;

pub fn hunger_plugin(app: &mut App) {
  app
    .init_resource::<HungerEnabled>()
    .add_systems(
      Update,
      (process_hunger, update_hunger_indicators)
        .chain()
        .in_set(HungerTickSet),
    )
    .add_systems(Update, insert_hunger_on_agent_spawn)
    .add_systems(Update, insert_indicator_on_hunger_spawn)
    .add_observer(on_enable_hunger);
}

fn on_enable_hunger(
  _trigger: Trigger<EnableHunger>,
  q_agents: Query<Entity, With<Agent>>,
  mut r_hunger_enabled: ResMut<HungerEnabled>,
  mut commands: Commands,
) {
  if r_hunger_enabled.0 {
    return;
  }
  r_hunger_enabled.0 = true;

  for agent in q_agents.iter() {
    commands
      .entity(agent)
      .insert(Hunger::new(DEFAULT_HUNGER_CAPACITY));
  }
}

fn insert_hunger_on_agent_spawn(
  q_new_agents: Query<Entity, Added<Agent>>,
  r_hunger_enabled: Res<HungerEnabled>,
  mut commands: Commands,
) {
  if r_hunger_enabled.0 {
    for agent in q_new_agents.iter() {
      commands
        .entity(agent)
        .insert(Hunger::new(DEFAULT_HUNGER_CAPACITY));
    }
  }
}

fn process_hunger(mut q_agents: Query<(Entity, &mut Hunger), With<Agent>>, mut commands: Commands) {
  for (agent, mut hunger) in q_agents.iter_mut() {
    hunger.remaining -= 1;
    if hunger.remaining == 0 {
      info!("Oh dear, you are dead!");
      commands.entity(agent).despawn_recursive();
    }
  }
}

fn insert_indicator_on_hunger_spawn(
  q_agents_with_hunger: Query<Entity, Added<Hunger>>,
  mut r_meshes: ResMut<Assets<Mesh>>,
  mut r_materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  for agent in q_agents_with_hunger.iter() {
    commands.entity(agent).with_child((
      HungerIndicator,
      MeshMaterial2d(r_materials.add(Color::from(tw::RED_500))),
      // mesh is translated so that it scales from the side rather than from the center
      Mesh2d(r_meshes.add(Mesh::from(Rectangle::new(0.8, 0.15)).translated_by(Vec3::X * 0.4))),
      Transform::from_xyz(-0.4, 0.3, 0.1),
    ));
  }
}

fn update_hunger_indicators(
  q_agents_with_changed_hunger: Query<(&Hunger, &Children), Changed<Hunger>>,
  mut q_indicators: Query<&mut Transform, With<HungerIndicator>>,
) {
  for (hunger, children) in q_agents_with_changed_hunger.iter() {
    for &child in children.iter() {
      let Ok(mut indicator_transform) = q_indicators.get_mut(child) else {
        continue;
      };
      indicator_transform.scale.x = hunger.remaining as f32 / hunger.capacity as f32;
    }
  }
}

const DEFAULT_HUNGER_CAPACITY: usize = 10;

#[derive(Component)]
pub struct Hunger {
  remaining: usize,
  capacity: usize,
}

impl Hunger {
  pub fn new(capacity: usize) -> Self {
    Self {
      remaining: capacity,
      capacity,
    }
  }

  pub fn eat(&mut self, nutritional_value: usize) {
    self.remaining = (self.remaining + nutritional_value).clamp(0, self.capacity);
  }
}

#[derive(Component)]
struct HungerIndicator;

#[derive(Resource, Default)]
pub struct HungerEnabled(pub bool);

#[derive(Event)]
pub struct EnableHunger;
