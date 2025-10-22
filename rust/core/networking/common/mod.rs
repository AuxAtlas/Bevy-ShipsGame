use bevy::platform::collections::HashMap;
use bevy_quinnet::shared::ClientId;
use godot_bevy::prelude::bevy_prelude::*;

#[derive(Debug, Clone, Default)]
pub struct NetUser {
    pub username: String,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct LobbyData {
    pub users: HashMap<ClientId, NetUser>,
}
