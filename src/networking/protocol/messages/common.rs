use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Packets {
    Hello {
        username: String,
    },
    LobbyInfo {
        users: HashMap<ClientId, String>,
    },
    LobbyInfoDelta {
        users_removed: Vec<ClientId>,
        users_added: HashMap<ClientId, String>,
    },
}