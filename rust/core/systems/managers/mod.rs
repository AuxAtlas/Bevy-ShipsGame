use crate::systems::managers::lobby::GameLobbyManagerPlugin;
use bevy::app::{App, Plugin};

mod lobby;

pub struct GameManagersPlugin;

impl Plugin for GameManagersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameLobbyManagerPlugin);
    }
}
