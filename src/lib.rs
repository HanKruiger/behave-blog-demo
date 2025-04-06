mod agent;
mod behaviours;
mod fruit;
mod glue;
mod grid;
mod hunger;
mod resizing;
mod schedule;

use agent::SpawnAgent;
use bevy::prelude::*;

use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgba(0.0, 0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      // wire up the Bevy rendering window as the Canvas in the DOM
      primary_window: Some(Window {
        canvas: Some("#behave-demo-canvas".into()),
        fit_canvas_to_parent: true,
        ..default()
      }),
      ..default()
    }))
    .add_plugins(EntropyPlugin::<WyRand>::default())
    // this app's plugins
    .add_plugins(glue::glue_plugin)
    .add_plugins(schedule::schedule_plugin)
    .add_plugins(resizing::resizing_plugin)
    .add_plugins(grid::grid_plugin)
    .add_plugins(agent::agent_plugin)
    .add_plugins(hunger::hunger_plugin)
    .add_plugins(behaviours::behaviours_plugin)
    .add_plugins(fruit::fruit_plugin)
    // main systems & observers
    .add_systems(Startup, setup)
    .add_observer(on_web_event)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn on_web_event(trigger: Trigger<glue::WebEvent>, mut commands: Commands) {
  match trigger.event() {
    glue::WebEvent::SpawnAgent => {
      commands.trigger(SpawnAgent);
    }
    glue::WebEvent::SetBehaviourWalkLeftRightNaive => {
      commands.trigger(behaviours::DisableNaiveMovementBehaviours);
      commands.trigger(behaviours::DisableMovementBehaviours);
      commands.trigger(behaviours::SetBehaviourWalkLeftRightNaive);
    }
    glue::WebEvent::SetBehaviourWalkLeftRight => {
      commands.trigger(behaviours::DisableNaiveMovementBehaviours);
      commands.trigger(behaviours::DisableMovementBehaviours);
      commands.trigger(behaviours::SetBehaviourWalkLeftRight);
    }
    glue::WebEvent::SetBehaviourMoveToClosestFruit => {
      commands.trigger(behaviours::DisableNaiveMovementBehaviours);
      commands.trigger(behaviours::DisableMovementBehaviours);
      commands.trigger(behaviours::SetBehaviourMoveToClosestFruit);
    }
    glue::WebEvent::SpawnFruitSpawner => {
      commands.trigger(fruit::SpawnFruitSpawner);
    }
    glue::WebEvent::EnableHunger => {
      commands.trigger(hunger::EnableHunger);
      commands.trigger(behaviours::EnableEating);
    }
  }
}
