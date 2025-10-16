use crate::entities::boat::BoatAssets;
use crate::resources::inputs::InputBuffer;
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingState, LoadingStateAppExt};

pub mod inputs;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::InGame)
                .load_collection::<BoatAssets>()
            )
            .init_resource::<InputBuffer>();
    }
}