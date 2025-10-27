use crate::messages::S2CPacketEvent;
use crate::networking::common::{LobbyData, NetUser};
use crate::networking::protocol::packets::Packets;
use crate::GameState;
use bevy::prelude::*;

pub struct GameLobbyManagerPlugin;

impl Plugin for GameLobbyManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            handle_new_players_system.run_if(in_state(GameState::InGame))
        );
    }
}

fn handle_new_players_system(
    mut packet_events_s2c: EventReader<S2CPacketEvent>,
    mut lobby_data: ResMut<LobbyData>,
) {
    for p in packet_events_s2c.read() {
        let packet = &p.0.packet;
        // let sender_id = &p.0.sender_id;
        match packet {
            Packets::LobbyInfo { users } => {
                lobby_data.users = users
                    .into_iter()
                    .map(|(k, v)| (*k, NetUser { username: v.clone() }))
                    .collect();
            }
            Packets::LobbyInfoDelta {
                users_added,
                users_removed,
            } => {
                for usr_id in users_removed {
                    if lobby_data.users.contains_key(usr_id) {
                        lobby_data.users.remove(usr_id);
                    } else {
                        println!("Attempted to remove non existing lobby user: {}", usr_id);
                    }
                }
                for usr in users_added {
                    let usr_id = *usr.0;
                    let usr_name = usr.1.clone();
                    if lobby_data.users.contains_key(usr.0) {
                        println!("Attempted to add an existing lobby user: {}", usr.0);
                    } else {
                        lobby_data.users.insert(usr_id, NetUser { username: usr_name });
                    }
                }
            }
            _ => println!(
                "[Handle S2C Message] Unknown Packet Received: {:?}",
                packet
            ),
        }
    }
}
