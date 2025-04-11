use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;

use crate::agent::Agent;

pub fn points_plugin(app: &mut App) {
  app.add_systems(
    Update,
    (
      insert_points_on_agent_spawn,
      insert_indicator_on_points_spawn,
      update_points_indicators,
    )
      .chain(),
  );
}

fn insert_points_on_agent_spawn(q_new_agents: Query<Entity, Added<Agent>>, mut commands: Commands) {
  for agent in q_new_agents.iter() {
    commands
      .entity(agent)
      .insert(Points::new(DEFAULT_POINTS_GOAL));
  }
}

fn insert_indicator_on_points_spawn(
  q_agents_with_points: Query<Entity, Added<Points>>,
  mut r_meshes: ResMut<Assets<Mesh>>,
  mut r_materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  for agent in q_agents_with_points.iter() {
    commands.entity(agent).with_child((
      PointsIndicator,
      MeshMaterial2d(r_materials.add(Color::from(tw::YELLOW_400))),
      // mesh is translated so that it scales from the side rather than from the center
      Mesh2d(r_meshes.add(Mesh::from(Rectangle::new(0.15, 0.8)).translated_by(Vec3::Y * 0.4))),
      Transform::from_xyz(0.3, -0.4, 0.1),
    ));
  }
}

fn update_points_indicators(
  q_agents_with_changed_points: Query<(&Points, &Children), Changed<Points>>,
  mut q_indicators: Query<&mut Transform, With<PointsIndicator>>,
) {
  for (points, children) in q_agents_with_changed_points.iter() {
    for &child in children.iter() {
      let Ok(mut indicator_transform) = q_indicators.get_mut(child) else {
        continue;
      };
      indicator_transform.scale.y = points.current as f32 / points.goal as f32;
    }
  }
}

const DEFAULT_POINTS_GOAL: usize = 10;

#[derive(Component)]
pub struct Points {
  current: usize,
  goal: usize,
}

impl Points {
  pub fn new(goal: usize) -> Self {
    Self { current: 0, goal }
  }

  pub fn earn(&mut self, monetary_value: usize) {
    self.current = (self.current + monetary_value).clamp(0, self.goal);
  }
}

#[derive(Component)]
struct PointsIndicator;
