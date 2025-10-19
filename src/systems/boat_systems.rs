use crate::components::stats::speed_stats::MoveSpeedStat;
use crate::gameplay::gameplay_boats::{Boat, BoatNode};
use crate::resources::input_resources::InputBuffer;
use crate::GameState;
use bevy::prelude::ops::sqrt;
use bevy::prelude::*;
use godot::classes::{Camera3D, CharacterBody3D};
use godot::prelude::*;
use godot_bevy::prelude::*;
use godot_bevy::utils::lerp;
use crate::components::maker_components::Puppet;

pub struct BoatSystemsPlugin;

impl Plugin for BoatSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PhysicsUpdate, move_boat_system.run_if(in_state(GameState::InGame)));
    }
}

#[main_thread_system]
pub fn move_boat_system(
    mut query: Query<(&mut GodotNodeHandle, &MoveSpeedStat, &mut Boat), (With<CharacterBody3DMarker>, With<Boat>, With<Puppet>)>,
    time: Res<Time>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    for (mut node_handle, speed_stat, mut boat) in query.iter_mut() {
        let mut character = node_handle.get::<CharacterBody3D>();

        let speed = speed_stat.base * speed_stat.multiplier * time.delta_secs();
        let movements = input_buffer.get_movements();

        let mut vel = character.get_velocity();
        vel.y = lerp(vel.y, 0.0, time.delta_secs());

        let forward_vel = movements.z * speed * -character.get_basis().col_c();
        vel += forward_vel;

        let vel_mag = sqrt(vel.x * vel.x + vel.z * vel.z);
        if(vel_mag > speed) {
            vel -= vel * (speed / 4.0);
        }
        if vel_mag > 0.0 {
            vel -= vel * speed / 12.0;
        }

        character.set_velocity(vel);
        character.rotate_object_local(Vector3::UP, -movements.x * time.delta_secs());

        character.move_and_slide();

        // Mouse/Look Inputs
        if boat.turret_socket_handle.is_some() {
           let mut turret_socket_node = boat.turret_socket_handle.as_mut().unwrap().get::<Node3D>();
            turret_socket_node.rotate_object_local(Vector3::UP, input_buffer.look_delta.x * time.delta_secs());
            turret_socket_node.rotate_object_local(Vector3::RIGHT, input_buffer.look_delta.y * time.delta_secs());
            input_buffer.look_delta = Vec2::ZERO;
        }
    }
}