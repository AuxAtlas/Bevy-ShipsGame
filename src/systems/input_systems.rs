use crate::resources::input_resources::InputBuffer;
use crate::{GameState, GameSystems};
use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub struct InputSystemsPlugin;

impl Plugin for InputSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_inputs_system.run_if(in_state(GameState::InGame)));
    }
}

pub fn read_inputs_system(
    mut action_input_events: EventReader<ActionInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut input_buffer: ResMut<InputBuffer>,
) {

    // Mouse Events
    for event in mouse_motion_events.read() {
        input_buffer.look_delta += event.delta;
    }
    
    for event in action_input_events.read() {
        let action_bool_flag = event.pressed || !event.strength.is_zero_approx();
        match event.action.as_str() {
            "move_left" => {
                input_buffer.flags.left = action_bool_flag;
            }
            "move_right" => {
                input_buffer.flags.right = action_bool_flag;
            }
            "move_forward" => {
                input_buffer.flags.forward = action_bool_flag;
            }
            "move_backward" => {
                input_buffer.flags.backward = action_bool_flag;
            }
            "move_up" => {
                input_buffer.flags.up = action_bool_flag;
            }
            "move_down" => {
                input_buffer.flags.down = action_bool_flag;
            }

            _ => {}
        }
    }
}