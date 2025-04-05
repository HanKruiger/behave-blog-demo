use bevy::{prelude::*, utils::HashMap};

use gloo::events::EventListener;

pub struct GluePlugin;

impl Plugin for GluePlugin {
  fn build(&self, app: &mut App) {
    // create a channel for communication between web event listeners and Bevy
    let (sender, receiver) = crossbeam_channel::unbounded::<WebEvent>();

    // insert channel sender and receiver as separate resources
    app.insert_resource(GlueSender(sender));
    app.insert_resource(GlueReceiver(receiver));

    app.add_systems(Startup, wire_up_buttons);
    app.add_systems(Update, forward_web_events);
  }
}

/// attach click listeners to button elements, and sends them to the channel
/// (it is not possible to directly send to Bevy from the closure)
fn wire_up_buttons(sender: Res<GlueSender<WebEvent>>) {
  let mut button_click_mapping = HashMap::new();
  button_click_mapping.insert("spawn", WebEvent::SpawnAgent);
  button_click_mapping.insert("walk-lr-naive", WebEvent::SetBehaviourWalkLeftRightNaive);
  button_click_mapping.insert("walk-lr", WebEvent::SetBehaviourWalkLeftRight);

  let window = web_sys::window().expect("could not get window from web_sys");
  let document = window.document().expect("could not get document");

  for (id, event) in button_click_mapping.iter() {
    let dom_button = document
      .query_selector(&format!("button#{}", id))
      .expect("query selector failed")
      .expect("element not found");

    let sender_1 = sender.0.clone();
    let event_1 = event.clone();
    EventListener::new(&dom_button, "click", move |_event| {
      sender_1.send(event_1).unwrap();
    })
    .forget();
  }
}

/// consumes WebEvents from the channel and forwards them to the Bevy trigger system
fn forward_web_events(receiver: ResMut<GlueReceiver<WebEvent>>, mut commands: Commands) {
  while let Ok(event) = receiver.0.try_recv() {
    commands.trigger(event);
  }
}

#[derive(Debug, Event, Clone, Copy)]
pub enum WebEvent {
  SpawnAgent,
  SetBehaviourWalkLeftRightNaive,
  SetBehaviourWalkLeftRight,
}

#[derive(Resource)]
struct GlueSender<T>(crossbeam_channel::Sender<T>);
#[derive(Resource)]
struct GlueReceiver<T>(crossbeam_channel::Receiver<T>);
