pub mod components;
mod systems;
pub mod gameplay;
pub mod resources;
pub mod networking;

use crate::gameplay::GameplayPlugin;
use crate::networking::client::NetClientPlugin;
use crate::networking::common::LobbyData;
use crate::networking::server::NetServerPlugin;
use crate::resources::ResourcesPlugin;
use crate::systems::SystemsPlugin;
use bevy::state::app::StatesPlugin;
use godot_bevy::prelude::bevy_prelude::{App, AppExtStates, States, SystemSet};
use godot_bevy::prelude::{
	godot_prelude::{gdextension, ExtensionLibrary},
	GodotDefaultPlugins,
	*,
};

#[bevy_app]
fn build_app(app: &mut App) {
	// GodotDefaultPlugins provides all standard godot-bevy functionality
	app.add_plugins(GodotDefaultPlugins)
		.init_resource::<LobbyData>()
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
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
