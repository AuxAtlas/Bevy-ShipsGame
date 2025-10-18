use bevy::prelude::Resource;
use bevy_quinnet::shared::ClientId;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct NetUser {
    pub username: String,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct LobbyData {
    pub users : HashMap<ClientId, NetUser>
}