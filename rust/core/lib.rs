use crate::gameplay::GameplayPlugin;
use crate::networking::common::LobbyData;
use crate::networking::GameNetworkingPlugin;
use crate::resources::GameResourcesPlugin;
use crate::systems::GameSystemsPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub mod components;
mod gameplay;
pub mod messages;
pub mod networking;
pub mod resources;
mod systems;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins);

    app.init_resource::<LobbyData>()
        .add_plugins((
            StatesPlugin,
            GameSystemsPlugin,
            GameResourcesPlugin,
            GameplayPlugin,
            GameNetworkingPlugin,
        ))
        .init_state::<GameState>();
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSystems {
    HostSystems,
    ClientSystems,
}
