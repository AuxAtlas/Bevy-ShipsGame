use crate::components::stats::health::HealthStat;
use crate::components::stats::speed::MoveSpeedStat;
use bevy_asset_loader::asset_collection::AssetCollection;
use godot::classes::CharacterBody3D;
use godot::prelude::*;
use godot_bevy::prelude::bevy_prelude::*;
use godot_bevy::prelude::{BevyBundle, GodotResource};

#[derive(GodotClass, BevyBundle)]
#[class(init, base=CharacterBody3D)]
#[bevy_bundle(
    (MoveSpeedStat { base: base_speed, multiplier: speed_multiplier}),
    (HealthStat{ current: health, maximum: max_health }),
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
}

#[derive(Component, Default)]
pub struct Boat;

#[derive(AssetCollection, Resource, Debug)]
#[derive(Default)]
pub struct BoatAssets {
    #[asset(path = "templates/entities/ships/DevBoatEntity.tscn")]
    pub boat_scene: Handle<GodotResource>,
}