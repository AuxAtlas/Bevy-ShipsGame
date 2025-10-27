use bevy::prelude::*;
use crate::networking::protocol::packets::Packets;

#[derive(Event)]
pub struct C2SPacketEvent(pub PacketData);

#[derive(Event)]
pub struct S2CPacketEvent(pub PacketData);

pub struct PacketData {
    pub sender_id: u64,
    pub packet: Packets,
}
