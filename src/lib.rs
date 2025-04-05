#![no_std]

mod glue;
mod grid;
mod resizing;

use bevy::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgba(0.0, 0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        canvas: Some("#behave-demo-canvas".into()),
        fit_canvas_to_parent: true,
        ..default()
      }),
      ..default()
    }))
    .add_plugins(glue::GluePlugin)
    .add_plugins(resizing::ResizingPlugin)
    .add_plugins(grid::GridPlugin)
    .add_systems(Startup, setup)
    .add_observer(on_web_event)
    .run();
}

fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2d);
}

fn on_web_event(trigger: Trigger<glue::WebEvent>, mut _commands: Commands) {
  match trigger.event() {
    glue::WebEvent::SpawnThing => {}
  }
}
