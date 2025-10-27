use crate::networking::protocol::messages::common_messages::Packets;
use bevy::prelude::*;

#[derive(Event)]
pub struct C2SPacketEvent(pub PacketData);

#[derive(Event)]
pub struct S2CPacketEvent(pub PacketData);

pub struct PacketData {
    pub sender_id: u64,
    pub packet: Packets,
}
