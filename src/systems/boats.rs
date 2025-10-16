use crate::components::stats::speed::MoveSpeedStat;
use crate::entities::boat::Boat;
use crate::resources::inputs::InputBuffer;
use crate::GameState;
use bevy::prelude::ops::sqrt;
use bevy::prelude::*;
use godot::classes::CharacterBody3D;
use godot::prelude::*;
use godot_bevy::prelude::*;
use godot_bevy::utils::lerp;

pub struct BoatSystemsPlugin;

impl Plugin for BoatSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PhysicsUpdate, move_boat.run_if(in_state(GameState::InGame)));
    }
}

#[main_thread_system]
pub fn move_boat(
    mut query: Query<(&mut GodotNodeHandle, &MoveSpeedStat), (With<CharacterBody3DMarker>, With<Boat>)>,
    time: Res<Time>,
    input_buffer: Res<InputBuffer>) {
    for (mut handle, speed_stat) in query.iter_mut() {
        let mut character = handle.get::<CharacterBody3D>();

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
    }
}