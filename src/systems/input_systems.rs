use crate::resources::input_resources::InputBuffer;
use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub fn inputs_system(
    mut events: EventReader<ActionInput>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let mut movement_input = input_buffer.movement;
    
    for event in events.read() {
        match event.action.as_str() {
            "move_left" => {
                movement_input.x = -event.strength;
            }
            "move_right" => {
                movement_input.x = event.strength;
            }
            "move_forward" => {
                movement_input.z = -event.strength;
            }
            "move_backward" => {
                movement_input.z = event.strength;
            }
            "move_up" => {
                movement_input.y = event.strength;
            }
            "move_down" => {
                movement_input.y = -event.strength;
            }

            _ => {}
        }
    }

    input_buffer.movement = movement_input.normalized_or_zero();
}