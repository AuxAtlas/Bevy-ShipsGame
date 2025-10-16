use bevy::prelude::*;

#[derive(Component)]
pub struct HealthStat {
    pub current: f32,
    pub maximum: f32,
}

impl Default for HealthStat {
    fn default() -> Self {
        HealthStat {
            current: 100.0,
            maximum: 100.0,
        }
    }
}