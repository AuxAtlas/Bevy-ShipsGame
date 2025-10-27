use crate::messages::C2SPacketEvent;
use crate::networking::client::NetClientPlugin;
use crate::networking::server::NetServerPlugin;
use bevy::prelude::*;

pub(crate) mod client;
pub(crate) mod common;
pub(crate) mod protocol;
pub(crate) mod server;

pub struct GameNetworkingPlugin;

impl Plugin for GameNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NetClientPlugin, NetServerPlugin));
        app.add_event::<C2SPacketEvent>();
    }
}
