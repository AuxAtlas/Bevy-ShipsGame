use crate::resources::inputs::InputBuffer;
use crate::GameState;
use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub struct InputSystemsPlugin;

impl Plugin for InputSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, read_inputs.run_if(in_state(GameState::InGame)));
    }
}

pub fn read_inputs(
    mut events: EventReader<ActionInput>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    for event in events.read() {
        match event.action.as_str() {
            "move_left" => {
                if !input_buffer.input_flags.left && (event.pressed || !event.strength.is_zero_approx()) {
                    input_buffer.input_flags.left = true;
                }
                else if input_buffer.input_flags.left && (!event.pressed && event.strength.is_zero_approx()) {
                    input_buffer.input_flags.left = false;
                }
            }
            "move_right" => {
                if !input_buffer.input_flags.right && (event.pressed || !event.strength.is_zero_approx()) {
                    input_buffer.input_flags.right = true;
                }
                else if input_buffer.input_flags.right && (!event.pressed && event.strength.is_zero_approx()) {
                    input_buffer.input_flags.right = false;
                }
            }
            "move_forward" => {
                if !input_buffer.input_flags.forward && (event.pressed || !event.strength.is_zero_approx()) {
                    input_buffer.input_flags.forward = true;
                }
                else if input_buffer.input_flags.forward && (!event.pressed && event.strength.is_zero_approx()) {
                    input_buffer.input_flags.forward = false;
                }
            }
            "move_backward" => {
                if !input_buffer.input_flags.backward && (event.pressed || !event.strength.is_zero_approx()) {
                    input_buffer.input_flags.backward = true;
                }
                else if input_buffer.input_flags.backward && (!event.pressed && event.strength.is_zero_approx()) {
                    input_buffer.input_flags.backward = false;
                }
            }
            "move_up" => {
                if !input_buffer.input_flags.up && (event.pressed || !event.strength.is_zero_approx()) {
                    input_buffer.input_flags.up = true;
                }
                else if input_buffer.input_flags.up && (!event.pressed && event.strength.is_zero_approx()) {
                    input_buffer.input_flags.up = false;
                }
            }
            "move_down" => {
                if !input_buffer.input_flags.down && (event.pressed || !event.strength.is_zero_approx()) {
                    input_buffer.input_flags.down = true;
                }
                else if input_buffer.input_flags.down && (!event.pressed && event.strength.is_zero_approx()) {
                    input_buffer.input_flags.down = false;
                }
            }

            _ => {}
        }
    }

}