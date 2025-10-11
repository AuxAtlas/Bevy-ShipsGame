use godot::prelude::*;
use bevy::prelude::*;
use godot_bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct MoveSpeedStat {
    pub base_speed : f32,
    pub multiplier: f32,
}

impl Default for MoveSpeedStat {
    fn default() -> Self {
        MoveSpeedStat {
            base_speed: 8.0,
            multiplier: 1.0,
        }
    }
}

