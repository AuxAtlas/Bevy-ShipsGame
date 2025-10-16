use crate::components::stats::speed::MoveSpeedStat;
use crate::entities::boat_entity::Boat;
use crate::resources::input_resources::InputBuffer;
use bevy::prelude::*;
use godot::classes::CharacterBody3D;
use godot::prelude::*;
use godot_bevy::prelude::*;
use crate::DebugRemovable;

#[main_thread_system]
pub fn move_boat(
    mut query: Query<(&mut GodotNodeHandle), With<DebugRemovable>>,
    time: Res<Time>,
    input_buffer: Res<InputBuffer>) {
    for (mut handle, speed_stat) in query.iter_mut() {
        let mut character = handle.get::<CharacterBody3D>();

        let gravity_vel = character.get_velocity().y - (character.get_gravity().y) * time.delta_secs();
        let speed = speed_stat.base * speed_stat.multiplier * time.delta_secs();
        character.set_velocity(Vector3::new(input_buffer.movement.x * speed, gravity_vel, input_buffer.movement.z * speed));
    }
}