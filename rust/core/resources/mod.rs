use crate::gameplay::gameplay_boats::BoatAssets;
use crate::resources::input_resources::InputBuffer;
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod input_resources;

pub(crate) struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::InGame)
                .load_collection::<BoatAssets>(),
        )
        .init_resource::<InputBuffer>();
    }
}
