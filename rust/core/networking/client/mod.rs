use crate::messages::{PacketData, S2CPacketEvent};
use crate::{GameState, GameSystems};
use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::{ClientAddrConfiguration, ConnectionFailedEvent};
use bevy_quinnet::client::{ClientConnectionConfiguration, QuinnetClient, QuinnetClientPlugin};
use bevy_quinnet::server::{ConnectionEvent, ConnectionLostEvent};
use crate::networking::protocol::packets::Packets;

pub struct NetClientPlugin;

impl Plugin for NetClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetClientPlugin::default());

        app.add_systems(OnEnter(GameState::InGame), start_connection);

        app.edit_schedule(Update, |schedule| {
            schedule.configure_sets(
                GameSystems::ClientSystems
                    .run_if(in_state(GameState::InGame))
                    .run_if(client_is_connected),
            );
            schedule.add_systems(
                ((
                    handle_s2c_messages_system,
                    client_handle_network_events_system,
                )
                    .chain(),)
                    .in_set(GameSystems::ClientSystems),
            );
        });
    }
}

// Functions
fn start_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(ClientConnectionConfiguration {
            addr_config: ClientAddrConfiguration::from_strings("[::1]:7777", "[::]:0").unwrap(),
            cert_mode: CertificateVerificationMode::SkipVerification,
            defaultables: Default::default(),
        })
        .expect("Need a valid server address.");
}
fn client_is_connected(client: Res<QuinnetClient>) -> bool {
    client.is_connected()
}

// Systems
fn handle_s2c_messages_system(
    mut client: ResMut<QuinnetClient>,
    mut packet_events_s2c: EventWriter<S2CPacketEvent>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Some(message) = connection.try_receive_message() {
            packet_events_s2c.write(S2CPacketEvent(PacketData {
                sender_id: 0,
                packet: message,
            }));
        }
    }
}
fn client_handle_network_events_system(
    mut connection_events: EventReader<ConnectionEvent>,
    mut disconnection_events: EventReader<ConnectionLostEvent>,
    mut connection_failed_events: EventReader<ConnectionFailedEvent>,
    mut client: ResMut<QuinnetClient>,
) {
    for _ in connection_events.read() {
        let username: String = "DEV_PLAYER".to_string();

        println!(
            "Connection to server! Sharing our username as {}({})",
            username,
            client.connection().client_id().unwrap()
        );
        client
            .connection_mut()
            .send_message(Packets::Hello { username })
            .unwrap();
    }

    for _ in disconnection_events.read() {
        println!("Connection lost.");
    }
    for ev in connection_failed_events.read() {
        println!(
            "Failed to connect: {:?}, make sure the chat-server is running.",
            ev.err
        );
    }
}
