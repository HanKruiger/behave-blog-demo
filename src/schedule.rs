use core::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
  fn build(&self, app: &mut App) {
    // configure a system set that runs every `TICK_DURATION`
    app.configure_sets(Update, (TickSet,).chain().run_if(on_timer(TICK_DURATION)));
  }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TickSet;

const TICK_DURATION: Duration = Duration::from_millis(600);
