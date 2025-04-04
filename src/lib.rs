#![no_std]

mod glue;

use bevy::{color::palettes::basic::*, prelude::*};

use glue::{GluePlugin, WebEvent};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
  App::new()
    .insert_resource(ClearColor(Color::srgba(0.0, 0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        // provide the ID selector string here
        canvas: Some("#behave-demo-canvas".into()),
        // ... any other window properties ...
        ..default()
      }),
      ..default()
    }))
    .add_plugins(GluePlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, button_system)
    .add_observer(on_web_event)
    .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
  mut interaction_query: Query<
    (
      &Interaction,
      &mut BackgroundColor,
      &mut BorderColor,
      &Children,
    ),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, mut border_color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Pressed => {
        **text = "Press".to_string();
        *color = PRESSED_BUTTON.into();
        border_color.0 = RED.into();
      }
      Interaction::Hovered => {
        **text = "Hover".to_string();
        *color = HOVERED_BUTTON.into();
        border_color.0 = Color::WHITE;
      }
      Interaction::None => {
        **text = "Button".to_string();
        *color = NORMAL_BUTTON.into();
        border_color.0 = Color::BLACK;
      }
    }
  }
}

fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2d);
}

fn on_web_event(trigger: Trigger<WebEvent>, mut commands: Commands) {
  match trigger.event() {
    WebEvent::SpawnThing => {
      commands.spawn(button());
    }
  }
}

fn button() -> impl Bundle + use<> {
  (
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      ..default()
    },
    children![(
      Button,
      Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
      },
      BorderColor(Color::BLACK),
      BorderRadius::MAX,
      BackgroundColor(NORMAL_BUTTON),
      children![(
        Text::new("Button"),
        TextFont {
          font_size: 33.0,
          ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextShadow::default(),
      )]
    )],
  )
}
