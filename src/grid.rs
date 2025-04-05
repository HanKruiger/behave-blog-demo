use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;

use crate::resizing::{CellSize, CellSizeChanged, GridBounds, GridSizeChanged};

pub struct GridPlugin;

impl Plugin for GridPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_observer(spawn_grid)
      .add_observer(resize_and_translate_on_cell_size_changed)
      .add_systems(Update, translate_moved_cells);
  }
}

fn spawn_grid(
  _trigger: Trigger<GridSizeChanged>,
  r_grid_bounds: Res<GridBounds>,
  q_background_cells: Query<Entity, With<Ground>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  // despawn old grid
  for e in q_background_cells.iter() {
    commands.entity(e).despawn();
  }

  // spawn new grid
  for x in r_grid_bounds.left_inclusive()..r_grid_bounds.right_exclusive() {
    for y in r_grid_bounds.top_inclusive()..r_grid_bounds.bottom_exclusive() {
      commands.spawn((
        GridCell::new(x, y),
        Ground,
        Mesh2d(meshes.add(Rectangle::new(0.8, 0.8))),
        MeshMaterial2d(materials.add(Color::from(tw::GRAY_600))),
      ));
    }
  }
  commands.trigger(CellSizeChanged);
}

fn translate_moved_cells(
  mut q_moved: Query<(&GridCell, &mut Transform), Changed<GridCell>>,
  r_grid_bounds: Res<GridBounds>,
  r_cell_size: Res<CellSize>,
) {
  let oddness = r_grid_bounds.oddness();
  for (cell, mut transform) in q_moved.iter_mut() {
    transform.translation = Vec3::from((
      Vec2::from(layout(cell.into(), r_cell_size.0, oddness)),
      transform.translation.z,
    ));
    transform.scale = Vec3::ONE * r_cell_size.0;
  }
}

fn resize_and_translate_on_cell_size_changed(
  _trigger: Trigger<CellSizeChanged>,
  mut q_cells: Query<(&GridCell, &mut Transform)>,
  r_grid_bounds: Res<GridBounds>,
  r_cell_size: Res<CellSize>,
) {
  let oddness = r_grid_bounds.oddness();
  for (cell, mut transform) in q_cells.iter_mut() {
    transform.translation = Vec3::from((
      Vec2::from(layout(cell.into(), r_cell_size.0, oddness)),
      transform.translation.z,
    ));
    transform.scale = Vec3::ONE * r_cell_size.0;
  }
}

fn layout(coordinates: (isize, isize), cell_size: f32, oddness: (bool, bool)) -> (f32, f32) {
  (
    ((coordinates.0 as f32 + 0.5) * cell_size) as f32
      - if oddness.0 { 0.5 * cell_size } else { 0.0 },
    ((coordinates.1 as f32 + 0.5) * cell_size) as f32
      - if oddness.1 { 0.5 * cell_size } else { 0.0 },
  )
}

#[derive(Component, Default)]
#[require(Transform)]
pub struct GridCell {
  pub x: isize,
  pub y: isize,
}

impl GridCell {
  pub fn new(x: isize, y: isize) -> Self {
    Self { x, y }
  }
}

impl From<&GridCell> for (isize, isize) {
  fn from(value: &GridCell) -> Self {
    (value.x, value.y)
  }
}

#[derive(Component)]
pub struct Ground;
