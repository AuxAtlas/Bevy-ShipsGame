// use bevy_quinnet::shared::channels::ChannelId;
// use strum_macros::EnumIter;
//
// #[derive(Debug, Clone, Copy, EnumIter)]
// #[repr(u8)]
// pub enum NetChannels {
//     GameSetup,
//     WorldEvents,
//     EntityCritical,
//     EntityNonCritical,
// }
//
// impl Into<ChannelId> for NetChannels {
//     fn into(self) -> ChannelId {
//         self as ChannelId
//     }
// }
