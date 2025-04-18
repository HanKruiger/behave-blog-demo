use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;

use crate::grid::GridCell;

pub fn agent_plugin(app: &mut App) {
  app.add_observer(spawn_agent);
}

fn spawn_agent(
  _trigger: Trigger<SpawnAgent>,
  mut commands: Commands,
  mut r_meshes: ResMut<Assets<Mesh>>,
  mut r_materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn((
    Agent,
    Mesh2d(r_meshes.add(Rectangle::new(0.9, 0.9))),
    MeshMaterial2d(r_materials.add(Color::from(tw::GREEN_600))),
  ));
}

#[derive(Component)]
// bevy 0.16 syntax
// #[require(Transform::from_xyz(0.0, 0.0, 0.1), GridCell)]
#[require(Transform(|| Transform::from_xyz(0.0, 0.0, 0.1)), GridCell)]
pub struct Agent;

#[derive(Event)]
pub struct SpawnAgent;
