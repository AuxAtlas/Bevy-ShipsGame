use crate::components::stats::speed::MoveSpeedStat;
use crate::components::stats::health::HealthStat;
use godot::classes::CharacterBody3D;
use godot::prelude::*;
use godot_bevy::prelude::bevy_prelude::*;
use godot_bevy::prelude::{BevyBundle, GodotResource};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(GodotClass, BevyBundle)]
#[class(init, base=CharacterBody3D)]
#[bevy_bundle(
    (MoveSpeedStat { base: base_speed, multiplier: speed_multiplier}),
    (HealthStat{ current: health, maximum: max_health }),
)]
pub struct BoatNode {
    base: Base<CharacterBody3D>,

    #[export] base_speed: f32,
    #[export] speed_multiplier: f32,

    #[export] health: f32,
    #[export] max_health: f32,
}

#[derive(Component, Default)]
pub struct Boat;

#[derive(AssetCollection, Resource, Debug)]
#[derive(Default)]
pub struct BoatAssets {
    #[asset(path = "templates/entities/ships/DevBoatEntity.tscn")]
    pub boat_scene: Handle<GodotResource>,
}