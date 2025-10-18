use crate::networking::common::{LobbyData, NetUser};
use crate::networking::protocol::channels::common::NetChannels;
use crate::networking::protocol::messages::common::Packets;
use crate::{GameState, GameSystems};
use bevy::log::warn;
use bevy_quinnet::client::connection::ConnectionFailedEvent;
use bevy_quinnet::server::certificate::{CertificateRetrievalMode, ServerCertificate};
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent, EndpointAddrConfiguration, QuinnetServer, QuinnetServerPlugin, ServerEndpointConfiguration};
use godot_bevy::prelude::bevy_prelude::{in_state, App, EventReader, FixedUpdate, IntoScheduleConfigs, OnEnter, Plugin, Res, ResMut, Resource};
use std::collections::HashMap;

pub struct NetServerPlugin;
impl Plugin for NetServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default());

        app.edit_schedule(OnEnter(GameState::InGame), |schedule| {
            schedule.configure_sets
            (
              GameSystems::HostSystems
                  .run_if(in_state(GameState::InGame))
                  .run_if(server_is_listening)
            );
            schedule.add_systems(start_listening_system);
        });
        app.edit_schedule(FixedUpdate, |schedule| {
            schedule.configure_sets
            (
                GameSystems::HostSystems
                    .run_if(in_state(GameState::InGame))
                    .run_if(server_is_listening)
            );
            schedule.add_systems
            (
                (
                    (
                        handle_c2s_messages_system,
                        server_handle_network_events_system,
                    )
                    .chain(),
                )
                .in_set(GameSystems::HostSystems),
            );
        });
    }
}



// Systems
fn handle_c2s_messages_system(
    mut server: ResMut<QuinnetServer>,
    mut lobby_data: ResMut<LobbyData>,
) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from(client_id, NetChannels::GameSetup) {
            match message {
                Packets::Hello { username } => {
                    if lobby_data.users.contains_key(&client_id) {
                        warn!("Received 'hello' packet from an existing client id {}", client_id)
                    } else {
                        lobby_data.users.insert(client_id, NetUser { username: username.clone(), });
                        println!("User '{}' joined.", username);
                        endpoint.send_message_on(
                            client_id,
                            NetChannels::GameSetup,
                    Packets::LobbyInfo {users: lobby_data.users.clone().into_iter().map(|(k,v)| (k, v.username)).collect()}
                        ).unwrap();
                        
                        endpoint.send_group_message_on(
                            lobby_data.users.keys().filter(|&x| x != &client_id),
                            NetChannels::GameSetup,
                            Packets::LobbyInfoDelta {
                                users_removed: vec![],
                                users_added: HashMap::from([(client_id, username)]),
                            }
                        ).unwrap();
                    }
                }
                _ => println!("[Handle C2S Message] Unknown Packet Received: {:?}", message),
            }
        }
    }
}

fn server_handle_network_events_system(
    mut connection_events: EventReader<ConnectionEvent>,
    mut disconnection_events: EventReader<ConnectionLostEvent>,
    mut connection_failed_events: EventReader<ConnectionFailedEvent>,
    mut lobby_data: ResMut<LobbyData>,
) {
    if !connection_events.is_empty() {
        for ev in connection_events.read() {
            println!("Connection Attempt from ClientId: {:?}", &ev.id);
        }

        connection_events.clear();
    }

    if !disconnection_events.is_empty() {
        for ev in disconnection_events.read() {
            lobby_data.users.remove_entry(&ev.id);
            println!( "Lost connection to client: {:?}", &ev.id);
        }
    }


    for ev in connection_failed_events.read() {
        println!( "Failed to connect: {:?}", ev.err);
    }
}

fn server_is_listening(server: Res<QuinnetServer>) -> bool {
    server.is_listening()
}

fn start_listening_system(mut server: ResMut<QuinnetServer>) {
    server.start_endpoint(ServerEndpointConfiguration {
        addr_config: EndpointAddrConfiguration::from_string("[::]:7777").unwrap(),
        cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
            server_hostname: "::1".to_owned(),
        },
        defaultables: Default::default(),
    }).expect("Requires a valid endpoint configuration to start the server listener.");
}