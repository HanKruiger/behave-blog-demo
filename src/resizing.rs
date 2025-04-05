use bevy::{prelude::*, window::WindowResized};

use crate::grid::GridCell;

pub struct ResizingPlugin;

impl Plugin for ResizingPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<GridBounds>()
      .init_resource::<CellSize>()
      .add_systems(Update, resize_grid_on_resize);
  }
}

fn resize_grid_on_resize(
  mut resize_reader: EventReader<WindowResized>,
  mut r_grid_bounds: ResMut<GridBounds>,
  mut r_cell_size: ResMut<CellSize>,
  mut commands: Commands,
) {
  for e in resize_reader.read() {
    let (grid_bounds, cell_size) =
      compute_grid_bounds_for_available_space(e.width.round() as usize, e.height.round() as usize);
    if *r_grid_bounds != grid_bounds {
      *r_grid_bounds = grid_bounds;
      *r_cell_size = cell_size;
      commands.trigger(GridSizeChanged);
      // GridSizeChanged observer also triggers CellSizeChanged, so no need to trigger it here.
    } else if *r_cell_size != cell_size {
      *r_cell_size = cell_size;
      commands.trigger(CellSizeChanged);
    }
  }
}

#[derive(Event)]
pub struct GridSizeChanged;
#[derive(Event)]
pub struct CellSizeChanged;

/// Represents the number of rows / columns of cells should exist.
#[derive(Resource, Default, PartialEq, Debug)]
pub struct GridBounds {
  width: usize,
  height: usize,
}

#[derive(Resource, Default, PartialEq, Debug)]
pub struct CellSize(pub f32);

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
}

const CELL_SIZE_IDEAL: usize = 50;
const MIN_HORIZONTAL_CELLS: usize = 10;
const MIN_VERTICAL_CELLS: usize = 10;

fn compute_grid_bounds_for_available_space(width: usize, height: usize) -> (GridBounds, CellSize) {
  let n_horizontal_cells_using_ideal = width / CELL_SIZE_IDEAL;
  let n_vertical_cells_using_ideal = height / CELL_SIZE_IDEAL;

  if n_horizontal_cells_using_ideal < MIN_HORIZONTAL_CELLS
    || n_vertical_cells_using_ideal < MIN_VERTICAL_CELLS
  {
    if n_horizontal_cells_using_ideal as f32 / (MIN_HORIZONTAL_CELLS as f32)
      < n_vertical_cells_using_ideal as f32 / (MIN_VERTICAL_CELLS as f32)
    {
      // horizontal direction is more limiting
      let cell_size_horizontal_fits = width / MIN_HORIZONTAL_CELLS;
      let n_vertical_cells_horizontal_fits = height / cell_size_horizontal_fits;
      (
        GridBounds::from_size(MIN_HORIZONTAL_CELLS, n_vertical_cells_horizontal_fits),
        CellSize(cell_size_horizontal_fits as f32),
      )
    } else {
      // vertical direction is more limiting
      let cell_size_vertical_fits = height / MIN_VERTICAL_CELLS;
      let n_horizontal_cells_vertical_fits = width / cell_size_vertical_fits;
      (
        GridBounds::from_size(n_horizontal_cells_vertical_fits, MIN_VERTICAL_CELLS),
        CellSize(cell_size_vertical_fits as f32),
      )
    }
  } else {
    (
      GridBounds::from_size(n_horizontal_cells_using_ideal, n_vertical_cells_using_ideal),
      CellSize(CELL_SIZE_IDEAL as f32),
    )
  }
}
