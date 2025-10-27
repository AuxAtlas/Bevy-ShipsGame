use bevy::prelude::*;

#[derive(Component)]
pub struct MoveSpeedStat {
    pub base: f32,
    pub multiplier: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

impl Default for MoveSpeedStat {
    fn default() -> Self {
        MoveSpeedStat {
            base: 8.0,
            multiplier: 1.0,
            acceleration: 4.0,
            deceleration: 4.0,
        }
    }
}
