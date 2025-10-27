use crate::components::stats::health_stats::HealthStat;
use crate::components::stats::speed_stats::MoveSpeedStat;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::classes::{Camera3D, CharacterBody3D};
use godot::prelude::*;
use godot_bevy::interop::GodotNodeHandle;
use godot_bevy::prelude::bevy_prelude::*;
use godot_bevy::prelude::{BevyBundle, GodotResource};
//
#[derive(GodotClass, BevyBundle)]
#[class(init, base=CharacterBody3D)]
#[bevy_bundle(
    (MoveSpeedStat { base: base_speed, multiplier: speed_multiplier, acceleration: acceleration, deceleration:deceleration }),
    (HealthStat { current: health, maximum: max_health }),
    (Boat { turret_camera_socket: turret_camera_socket, turret_pivot_socket: turret_pivot_socket }),
)]
pub struct BoatNode {
    base: Base<CharacterBody3D>,

    #[init(val = 100.0)]
    #[export]
    health: f32,

    #[init(val = 100.0)]
    #[export]
    max_health: f32,

    #[init(val = 5.0)]
    #[export]
    base_speed: f32,

    #[init(val = 1.0)]
    #[export]
    speed_multiplier: f32,

    #[init(val = 4.0)]
    #[export]
    acceleration: f32,

    #[init(val = 4.0)]
    #[export]
    deceleration: f32,

    #[export]
    #[bevy_bundle(transform_with = "node_to_handle")]
    pub turret_camera_socket: Option<Gd<Camera3D>>,

    #[export]
    #[bevy_bundle(transform_with = "node_to_handle")]
    pub turret_pivot_socket: Option<Gd<Node3D>>,
}

fn node_to_handle<T: GodotClass>(input: Option<Gd<T>>) -> Option<GodotNodeHandle> {
    if let Some(input) = input {
        return Some(GodotNodeHandle::from_instance_id(input.instance_id()));
    }
    None
}

#[derive(Component, Default)]
pub struct Boat {
    pub turret_camera_socket: Option<GodotNodeHandle>,
    pub turret_pivot_socket: Option<GodotNodeHandle>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct BoatAssets {
    #[asset(path = "res://templates/entities/ships/DevBoatEntity.tscn")]
    pub boat_scene: Handle<GodotResource>,
}
