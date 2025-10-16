use crate::entities::boat::{Boat, BoatAssets};
use crate::systems::debugging::DebugThisTransformMarker;
use crate::GameState;
use bevy::prelude::*;
use godot_bevy::prelude::GodotScene;

pub mod boat;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
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
        ));
}