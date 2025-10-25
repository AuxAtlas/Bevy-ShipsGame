use crate::components::maker_components::Puppet;
use crate::components::stats::speed_stats::MoveSpeedStat;
use crate::gameplay::gameplay_boats::Boat;
use crate::resources::input_resources::InputBuffer;
use crate::GameState;
use bevy::prelude::ops::sqrt;
use godot::classes::{Camera3D, CharacterBody3D};
use godot::prelude::*;
use godot_bevy::prelude::bevy_prelude::*;
use godot_bevy::prelude::*;
use godot_bevy::utils::lerp;

pub struct BoatSystemsPlugin;

impl Plugin for BoatSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsUpdate,
            move_boat_system.run_if(in_state(GameState::InGame)),
        );
    }
}

#[main_thread_system]
pub fn move_boat_system(
    mut query: Query<
        (&mut GodotNodeHandle, &MoveSpeedStat, &mut Boat),
        (With<CharacterBody3DMarker>, With<Boat>, With<Puppet>),
    >,
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
        if vel_mag > speed {
            vel -= vel * (speed / 4.0);
        }
        if vel_mag > 0.0 {
            vel -= vel * speed / 12.0;
        }

        character.set_velocity(vel);
        character.rotate_object_local(Vector3::UP, -movements.x * time.delta_secs());

        character.move_and_slide();

        let mut look_at_pos = Vector3::ZERO;
        // Mouse/Look Inputs
        if boat.turret_pivot_socket.is_some() {
            let mut turret_socket_node = boat.turret_pivot_socket.as_mut().unwrap().get::<Node3D>();
            turret_socket_node
                .global_rotate(Vector3::UP, input_buffer.look_delta.x * time.delta_secs());
            turret_socket_node.rotate_object_local(
                Vector3::RIGHT,
                input_buffer.look_delta.y * time.delta_secs(),
            );
            let mut turret_rot_clamped = turret_socket_node.get_rotation();
            turret_rot_clamped.x = turret_rot_clamped.x.clamp(-0.5, 0.5);
            turret_socket_node.set_rotation(turret_rot_clamped);
            look_at_pos = turret_socket_node.get_global_position();
        }
        if boat.turret_camera_socket.is_some() {
            let mut turret_camera_node = boat
                .turret_camera_socket
                .as_mut()
                .unwrap()
                .get::<Camera3D>();
            // turret_camera_node.rotate_object_local(Vector3::RIGHT, input_buffer.look_delta.y * time.delta_secs());
            turret_camera_node.look_at(look_at_pos);
        }
        input_buffer.look_delta = Vec2::ZERO;
    }
}
