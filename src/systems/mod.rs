use crate::systems::boats::BoatSystemsPlugin;
use crate::systems::debugging::DebuggingPlugin;
use crate::systems::inputs::InputSystemsPlugin;
use bevy::app::{App, Plugin};
use godot_bevy::prelude::bevy_prelude::IntoScheduleConfigs;

mod boats;
pub mod debugging;
mod inputs;

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