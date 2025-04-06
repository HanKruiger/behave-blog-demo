use crate::resizing::{CellSizeChanged, GridSizeChanged};
use bevy::color::palettes::tailwind as tw;
use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;

pub fn grid_plugin(app: &mut App) {
  app
    .init_resource::<GridBounds>()
    .init_resource::<CellSize>()
    .add_observer(spawn_grid)
    .add_observer(resize_and_translate_on_cell_size_changed)
    .add_systems(Update, translate_moved_cells);
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

#[derive(Component, Default, PartialEq, Copy, Clone)]
#[require(Transform)]
pub struct GridCell {
  pub x: isize,
  pub y: isize,
}

impl GridCell {
  pub fn new(x: isize, y: isize) -> Self {
    Self { x, y }
  }

  pub fn distance(&self, to: &GridCell) -> f32 {
    Vec2::from(self).distance(Vec2::from(to))
  }

  pub fn neighbours(&self) -> Vec<Self> {
    vec![
      GridCell::new(self.x - 1, self.y),
      GridCell::new(self.x + 1, self.y),
      GridCell::new(self.x, self.y - 1),
      GridCell::new(self.x, self.y + 1),
    ]
  }

  pub fn step_to(&mut self, target: &GridCell) {
    if (self.x - target.x).abs() > (self.y - target.y).abs() {
      self.x += (target.x - self.x) / (target.x - self.x).abs();
    } else {
      self.y += (target.y - self.y) / (target.y - self.y).abs();
    }
  }
}

impl From<&GridCell> for (isize, isize) {
  fn from(value: &GridCell) -> Self {
    (value.x, value.y)
  }
}

impl From<&GridCell> for Vec2 {
  fn from(value: &GridCell) -> Self {
    Vec2::new(value.x as f32, value.y as f32)
  }
}

#[derive(Component)]
pub struct Ground;

#[derive(Resource, Default, PartialEq, Debug)]
pub struct CellSize(pub f32);

/// Represents the number of rows / columns of cells should exist.
#[derive(Resource, Default, PartialEq, Debug)]
pub struct GridBounds {
  width: usize,
  height: usize,
}
impl GridBounds {
  pub fn from_size(width: usize, height: usize) -> Self {
    Self { width, height }
  }

  pub fn left_inclusive(&self) -> isize {
    -((self.width / 2) as isize)
  }

  pub fn right_exclusive(&self) -> isize {
    self.left_inclusive() + self.width as isize
  }

  pub fn top_inclusive(&self) -> isize {
    -((self.height / 2) as isize)
  }

  pub fn bottom_exclusive(&self) -> isize {
    self.top_inclusive() + self.height as isize
  }

  pub fn oddness(&self) -> (bool, bool) {
    (self.width % 2 != 0, self.height % 2 != 0)
  }

  pub fn contains(&self, grid_cell: &GridCell) -> bool {
    grid_cell.x >= self.left_inclusive()
      && grid_cell.x < self.right_exclusive()
      && grid_cell.y >= self.top_inclusive()
      && grid_cell.y < self.bottom_exclusive()
  }

  pub fn get_random_position(&self, rng: &mut GlobalEntropy<WyRand>) -> GridCell {
    GridCell::new(
      rng.gen_range(self.left_inclusive()..self.right_exclusive()),
      rng.gen_range(self.top_inclusive()..self.bottom_exclusive()),
    )
  }
}
