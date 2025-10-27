use crate::messages::{C2SPacketEvent, PacketData};
use crate::networking::common::LobbyData;
use crate::{GameState, GameSystems};
use bevy::prelude::*;
use bevy_quinnet::client::connection::ConnectionFailedEvent;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{
    ConnectionEvent, ConnectionLostEvent, EndpointAddrConfiguration, QuinnetServer,
    QuinnetServerPlugin, ServerEndpointConfiguration,
};

pub struct NetServerPlugin;
impl Plugin for NetServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default());

        app.edit_schedule(OnEnter(GameState::InGame), |schedule| {
            schedule.configure_sets(
                GameSystems::HostSystems
                    .run_if(in_state(GameState::InGame))
                    .run_if(server_is_listening),
            );
            schedule.add_systems(start_listening_system);
        });
        app.edit_schedule(Update, |schedule| {
            schedule.configure_sets(
                GameSystems::HostSystems
                    .run_if(in_state(GameState::InGame))
                    .run_if(server_is_listening),
            );
            schedule.add_systems(
                ((
                    handle_c2s_packets_system,
                    server_handle_network_events_system,
                )
                    .chain(),)
                    .in_set(GameSystems::HostSystems),
            );
        });
    }
}

// Systems
fn handle_c2s_packets_system(
    mut server: ResMut<QuinnetServer>,
    mut c2s_packet_event: EventWriter<C2SPacketEvent>,
) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message(client_id) {
            c2s_packet_event.write(C2SPacketEvent(PacketData {
                sender_id: client_id,
                packet: message,
            }));
        }
    }

    // while let Some(message) =
    //     endpoint.try_receive_message_from(client_id, NetChannels::GameSetup)
    // {
    //     match message {
    //         Packets::Hello { username } => {
    //             if lobby_data.users.contains_key(&client_id) {
    //                 warn!(
    //                     "Received 'hello' packet from an existing client id {}",
    //                     client_id
    //                 )
    //             } else {
    //                 lobby_data.users.insert(
    //                     client_id,
    //                     NetUser {
    //                         username: username.clone(),
    //                     },
    //                 );
    //                 println!("User '{}' joined.", username);
    //                 endpoint
    //                     .send_message_on(
    //                         client_id,
    //                         NetChannels::GameSetup,
    //                         Packets::LobbyInfo {
    //                             users: lobby_data
    //                                 .users
    //                                 .clone()
    //                                 .into_iter()
    //                                 .map(|(k, v)| (k, v.username))
    //                                 .collect(),
    //                         },
    //                     )
    //                     .unwrap();
    //
    //                 endpoint
    //                     .send_group_message_on(
    //                         lobby_data.users.keys().filter(|&x| x != &client_id),
    //                         NetChannels::GameSetup,
    //                         Packets::LobbyInfoDelta {
    //                             users_removed: vec![],
    //                             users_added: HashMap::from([(client_id, username)]),
    //                         },
    //                     )
    //                     .unwrap();
    //             }
    //         }
    //         _ => println!(
    //             "[Handle C2S Message] Unknown Packet Received: {:?}",
    //             message
    //         ),
    //     }
    // }
    // }
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
            println!("Lost connection to client: {:?}", &ev.id);
        }
    }

    for ev in connection_failed_events.read() {
        println!("Failed to connect: {:?}", ev.err);
    }
}

fn server_is_listening(server: Res<QuinnetServer>) -> bool {
    server.is_listening()
}

fn start_listening_system(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_string("[::]:7777").unwrap(),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "::1".to_owned(),
            },
            defaultables: Default::default(),
        })
        .expect("Requires a valid endpoint configuration to start the server listener.");
}
