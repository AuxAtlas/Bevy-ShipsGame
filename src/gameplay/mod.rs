use crate::gameplay::gameplay_boats::{Boat, BoatAssets};
use crate::systems::debug_systems::DebugThisTransformMarker;
use crate::GameState;
use bevy::prelude::*;
use godot_bevy::prelude::GodotScene;
use crate::components::maker_components::Puppet;

pub mod gameplay_boats;

pub(crate) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), initial_spawn);
    }
}

fn initial_spawn(mut commands: Commands, boat_assets: Res<BoatAssets>) {
    commands.spawn_empty()
        .insert(GodotScene::from_handle(boat_assets.boat_scene.clone()))
        .insert(Transform::default())
        .insert((
            Boat::default(),
            DebugThisTransformMarker,
            Puppet,
        ));
}