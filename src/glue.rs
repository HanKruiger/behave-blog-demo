use bevy::prelude::*;

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
  let window = web_sys::window().expect("could not get window from web_sys");
  let document = window.document().expect("could not get document");

  let dom_button = document
    .query_selector("#spawn-button")
    .expect("query selector failed")
    .expect("element not found");

  let sender_1 = sender.0.clone();

  EventListener::new(&dom_button, "click", move |_event| {
    sender_1.send(WebEvent::SpawnThing).unwrap();
  })
  .forget();
}

/// consumes WebEvents from the channel and forwards them to the Bevy trigger system
fn forward_web_events(receiver: ResMut<GlueReceiver<WebEvent>>, mut commands: Commands) {
  while let Ok(event) = receiver.0.try_recv() {
    commands.trigger(event);
  }
}

#[derive(Debug, Event)]
pub enum WebEvent {
  SpawnThing,
}

#[derive(Resource)]
struct GlueSender<T>(crossbeam_channel::Sender<T>);
#[derive(Resource)]
struct GlueReceiver<T>(crossbeam_channel::Receiver<T>);
