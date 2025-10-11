use godot::prelude::*;
use bevy::prelude::*;
use godot::classes::CharacterBody3D;
use godot_bevy::prelude::*;
use crate::components::marker_components::Boat;
use crate::components::stats_components::MoveSpeedStat;
use crate::resources::input_resources::InputBuffer;

pub fn move_boat(
    mut boat_nodes_query: Query<(&mut GodotNodeHandle), (With<CharacterBody3DMarker>, With<Boat>)>,
    stats_query: Query<(&GodotNodeHandle, &MoveSpeedStat), (With<CharacterBody3DMarker>, With<Boat>)>,
    time: Res<Time>,
    input_buffer: Res<InputBuffer>) {
    for mut handle in boat_nodes_query.iter_mut() {
        let mut character = handle.get::<CharacterBody3D>();
        let (_, move_speed_stat) = stats_query.iter().find(|&x| x.0.instance_id() == handle.instance_id()).expect("Boat found without a `base_speed`, and ``speed_multiplier` component(s) attached.");

        let gravity_vel = character.get_velocity().y - (character.get_gravity().y) * time.delta_secs();
        let speed = move_speed_stat.base_speed * move_speed_stat.multiplier * time.delta_secs();
        character.set_velocity(Vector3::new(input_buffer.movement.x * speed, gravity_vel, input_buffer.movement.z * speed));
    }
}