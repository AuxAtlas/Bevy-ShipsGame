mod components;
mod systems;
mod events;
mod resources;

use godot::prelude::*;
use bevy::prelude::*;
use godot_bevy::prelude::*;
use crate::systems::boat_systems::move_boat;
use crate::systems::input_systems::inputs_system;

#[bevy_app]
fn build_app(app: &mut App) {
	// GodotDefaultPlugins provides all standard godot-bevy functionality
	app.add_plugins(GodotDefaultPlugins);

	// Add your systems here
	app.add_systems(PhysicsUpdate, move_boat);
	
	app.add_systems(Update, inputs_system);
}
