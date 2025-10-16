pub mod components;
mod systems;
pub mod entities;
pub mod resources;

use crate::entities::EntitiesPlugin;
use crate::resources::ResourcesPlugin;
use crate::systems::SystemsPlugin;
use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_asset_loader::prelude::*;
use godot_bevy::prelude::{
	godot_prelude::{gdextension, ExtensionLibrary},
	GodotDefaultPlugins,
	*,
};

#[bevy_app]
fn build_app(app: &mut App) {
	// GodotDefaultPlugins provides all standard godot-bevy functionality
	app.add_plugins(GodotDefaultPlugins)
		.add_plugins((
	 		StatesPlugin,
			SystemsPlugin,
			ResourcesPlugin,
			EntitiesPlugin,
		))
		.init_state::<GameState>();
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
	#[default]
	Loading,
	InGame,
}
