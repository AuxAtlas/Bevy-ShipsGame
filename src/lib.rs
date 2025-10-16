mod components;
mod systems;
mod events;
mod resources;
mod entities;

use crate::systems::boat_systems::move_boat;
use crate::systems::input_systems::inputs_system;
use godot_bevy::prelude::{
	GodotDefaultPlugins,
	godot_prelude::{ExtensionLibrary, gdextension},
	*,
};
use bevy::{prelude::*, state::app::StatesPlugin};
use bevy_asset_loader::prelude::*;
use crate::entities::boat_entity::{Boat, BoatAssets};
use crate::resources::input_resources::InputBuffer;

#[bevy_app]
fn build_app(app: &mut App) {
	// GodotDefaultPlugins provides all standard godot-bevy functionality
	app.add_plugins(GodotDefaultPlugins)
		.add_plugins(StatesPlugin)
		.init_state::<GameState>()
		.add_loading_state(
			LoadingState::new(GameState::Loading)
				.continue_to_state(GameState::InGame)
				.load_collection::<BoatAssets>()
				.load_collection::<DebugGameAssets>()
	);

	app.init_resource::<InputBuffer>();

	// Add your systems here
	app.add_systems(PhysicsUpdate, move_boat);
	
	app.add_systems(Update, inputs_system);
	app.add_systems(OnExit(GameState::Loading), dev_setup);
}

#[derive(AssetCollection, Resource, Debug)]
#[derive(Default)]
pub struct DebugGameAssets {
	#[asset(path = "templates/DebuggerDrawer.tscn")]
	pub debugger_drawer_template: Handle<GodotResource>,
}

fn dev_setup(mut commands: Commands, boat_assets: Res<BoatAssets>, debug_assets: Res<DebugGameAssets>) {
	commands.spawn_empty()
		.insert(GodotScene::from_handle(boat_assets.boat_scene.clone()))
		.insert(Transform::default())
		.insert(Boat::default());

	commands.spawn_empty()
		.insert(GodotScene::from_handle(debug_assets.debugger_drawer_template.clone()))
		.insert(Transform::default())
		.insert(DebugRemovable);
}


#[derive(Component)]
pub struct DebugRemovable;
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
	#[default]
	Loading,
	InGame,
}
