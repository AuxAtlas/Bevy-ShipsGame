use godot::prelude::*;
use bevy::prelude::*;
use godot_bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub movement: Vector3,
    // pub look: Vector2,
}