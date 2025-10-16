use bevy::prelude::*;
use godot::prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub input_flags : MovementInputFlags,
}
impl InputBuffer {
    pub fn get_movements(&self) -> Vector3 {
        let mut movement = Vector3::ZERO;
        
        if(self.input_flags.forward) {
            movement.z = 1.0;
        }
        else if(self.input_flags.backward) {
            movement.z = -1.0;
        }
        
        if(self.input_flags.left) {
            movement.x = -1.0;
        }
        else if(self.input_flags.right) {
            movement.x = 1.0;
        }
        
        if(self.input_flags.up) {
            movement.y = 1.0;
        }
        else if(self.input_flags.down) {
            movement.y = -1.0;
        }
        
        movement
    }
}

#[derive(Default)]
pub struct MovementInputFlags {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}