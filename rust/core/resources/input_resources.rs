use godot::prelude::*;
use godot_bevy::prelude::bevy_prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub flags: MovementInputFlags,
    pub look_delta: Vec2,
}
impl InputBuffer {
    pub fn get_movements(&self) -> Vector3 {
        let mut movement = Vector3::ZERO;

        if self.flags.forward {
            movement.z = 1.0;
        } else if self.flags.backward {
            movement.z = -1.0;
        }

        if self.flags.left {
            movement.x = -1.0;
        } else if self.flags.right {
            movement.x = 1.0;
        }

        if self.flags.up {
            movement.y = 1.0;
        } else if self.flags.down {
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
