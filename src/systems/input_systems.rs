use godot::prelude::*;
use bevy::prelude::*;
use godot::classes::Input;
use godot_bevy::prelude::*;
use crate::resources::input_resources::InputBuffer;

pub fn inputs_system(
    mut events: EventReader<ActionInput>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let mut movement_input = Vector3::new(0.0, 0.0, 0.0);
    for event in events.read() {
        if event.pressed {
            match event.action.as_str() {
                "move_left" => {
                    movement_input.x = -1.0;
                }
                "move_right" => {
                    movement_input.x = 1.0;
                }
                "move_forward" => {
                    movement_input.z = -1.0;
                }
                "move_backward" => {
                    movement_input.z = 1.0;
                }
                "jump" => {
                    movement_input.y = 1.0;
                }
                "crouch" => {
                    movement_input.y = -1.0;
                }

                _ => {}
            }
        }
    }

    input_buffer.movement = movement_input;
}