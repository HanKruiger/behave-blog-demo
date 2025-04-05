mod agent;
mod behaviours;
mod glue;
mod grid;
mod resizing;
mod schedule;

use agent::SpawnAgent;
use bevy::prelude::*;

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
    // this app's plugins
    .add_plugins(glue::GluePlugin)
    .add_plugins(schedule::SchedulePlugin)
    .add_plugins(resizing::ResizingPlugin)
    .add_plugins(grid::GridPlugin)
    .add_plugins(agent::AgentPlugin)
    .add_plugins(behaviours::BehavioursPlugin)
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
      commands.trigger(behaviours::ClearNaiveBehaviours);
      commands.trigger(behaviours::ClearBehaviours);
      commands.trigger(behaviours::SetBehaviourWalkLeftRightNaive);
    }
    glue::WebEvent::SetBehaviourWalkLeftRight => {
      commands.trigger(behaviours::ClearNaiveBehaviours);
      commands.trigger(behaviours::ClearBehaviours);
      commands.trigger(behaviours::SetBehaviourWalkLeftRight);
    }
  }
}
