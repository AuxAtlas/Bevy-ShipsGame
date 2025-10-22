use bevy_quinnet::shared::channels::ChannelId;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u8)]
pub enum NetChannels {
    GameSetup,
    GameEvents,
}

impl Into<ChannelId> for NetChannels {
    fn into(self) -> ChannelId {
        self as ChannelId
    }
}

// impl NetChannels {
//     pub fn to_channel_config(self) -> ChannelConfig {
//         match self {
//             _ => ChannelConfig::default_ordered_reliable(),
//         }
//     }
//
//     pub fn channels_configuration() -> SendChannelsConfiguration {
//         SendChannelsConfiguration::from_configs(
//             NetChannels::iter()
//                 .map(NetChannels::to_channel_config)
//                 .collect(),
//         )
//         .unwrap()
//     }
// }
