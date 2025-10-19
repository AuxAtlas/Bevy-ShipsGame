use crate::systems::boat_systems::BoatSystemsPlugin;
use crate::systems::debug_systems::DebuggingPlugin;
use crate::systems::input_systems::InputSystemsPlugin;
use bevy::app::{App, Plugin};
use godot_bevy::prelude::bevy_prelude::IntoScheduleConfigs;

mod boat_systems;
pub mod debug_systems;
mod input_systems;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                InputSystemsPlugin,
                BoatSystemsPlugin,
                DebuggingPlugin,
            ));
    }
}