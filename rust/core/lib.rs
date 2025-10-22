use crate::gameplay::GameplayPlugin;
use crate::networking::client::NetClientPlugin;
use crate::networking::common::LobbyData;
use crate::networking::server::NetServerPlugin;
use crate::resources::ResourcesPlugin;
use crate::systems::SystemsPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub mod components;
pub mod gameplay;
pub mod networking;
pub mod resources;
pub(crate) mod systems;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins);

    app.init_resource::<LobbyData>()
        .add_plugins((
            StatesPlugin,
            SystemsPlugin,
            ResourcesPlugin,
            GameplayPlugin,
            NetServerPlugin,
            NetClientPlugin,
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
