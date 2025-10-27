use crate::systems::boat_systems::GameBoatSystemsPlugin;
use crate::systems::debug_systems::GameDebuggingPlugin;
use crate::systems::input_systems::GameInputSystemsPlugin;
use crate::systems::managers::GameManagersPlugin;
use godot_bevy::prelude::bevy_prelude::*;

mod boat_systems;
mod debug_systems;
mod input_systems;
mod managers;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameInputSystemsPlugin,
            GameBoatSystemsPlugin,
            GameDebuggingPlugin,
            GameManagersPlugin,
        ));
    }
}
