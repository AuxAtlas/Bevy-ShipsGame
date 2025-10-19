use crate::components::stats::health_stats::HealthStat;
use crate::components::stats::speed_stats::MoveSpeedStat;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::classes::{Camera3D, CharacterBody3D, Marker3D};
use godot::prelude::*;
use godot_bevy::interop::{GodotNodeHandle, Node3DMarker};
use godot_bevy::prelude::bevy_prelude::*;
use godot_bevy::prelude::{BevyBundle, GodotResource};
//
#[derive(GodotClass, BevyBundle)]
#[class(init, base=CharacterBody3D)]
#[bevy_bundle(
    (MoveSpeedStat { base: base_speed, multiplier: speed_multiplier}),
    (HealthStat { current: health, maximum: max_health }),
    (Boat { turret_socket_handle: turret_socket }),
)]
pub struct BoatNode {
    base: Base<CharacterBody3D>,

    #[init(val = 5.0)]
    #[export]
    base_speed: f32,

    #[init(val = 1.0)]
    #[export]
    speed_multiplier: f32,

    #[init(val = 100.0)]
    #[export]
    health: f32,

    #[init(val = 100.0)]
    #[export]
    max_health: f32,

    #[export]
    #[bevy_bundle(transform_with = "node3d_to_handle")]
    pub turret_socket: Option<Gd<Node3D>>
}

fn node3d_to_handle(input: Option<Gd<Node3D>>) -> Option<GodotNodeHandle> {
   if let Some(input) = input {
       Some(input);
   }
    None
}

#[derive(Component, Default)]
pub struct Boat {
    pub turret_socket_handle: Option<GodotNodeHandle>
}

#[derive(AssetCollection, Resource, Debug)]
#[derive(Default)]
pub struct BoatAssets {
    #[asset(path = "templates/entities/ships/DevBoatEntity.tscn")]
    pub boat_scene: Handle<GodotResource>,
}