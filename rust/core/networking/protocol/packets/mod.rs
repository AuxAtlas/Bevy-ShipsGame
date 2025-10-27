use std::collections::HashMap;
use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packets {
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

