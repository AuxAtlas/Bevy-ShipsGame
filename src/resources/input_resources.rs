use bevy::prelude::*;
use godot::prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub movement: Vector3,
    // pub look: Vector2,
}