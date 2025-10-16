use crate::resources::inputs::InputBuffer;
use crate::GameState;
use bevy::math::ops::sqrt;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingState, LoadingStateAppExt};
use godot::classes::{CharacterBody3D, Label, Node3D};
use godot_bevy::prelude::*;

pub struct DebuggingPlugin;
impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::InGame)
                    .load_collection::<DebugGameAssets>()
            )
            .add_systems(Update, update_debugger_text.run_if(in_state(GameState::InGame)))
            .add_systems(OnEnter(GameState::InGame), spawn_debug_tools);
    }
}

#[main_thread_system]
pub fn update_debugger_text(
                q_debugger_text_parents: Query<&Children, With<DebuggerText>>,
                mut q_debugger_text_children: Query<&mut GodotNodeHandle, With<LabelMarker>>,
                mut q_debug_this_transform : Query<&mut GodotNodeHandle, (With<DebugThisTransformMarker>, Without<LabelMarker>)>,
                input_buffer: Res<InputBuffer>){
    let mut debug_text: String = String::from("[DEBUG]\n");
    debug_text.push_str(format!("INPUT_MOVEMENT: {}\n", input_buffer.get_movements()).as_str());
    for(mut debug_node) in q_debug_this_transform.iter_mut() {
        let vel = debug_node.get::<CharacterBody3D>().get_velocity();
        let vel_mag = sqrt(vel.x * vel.x + vel.z * vel.z).round();
        debug_text.push_str(format!("(POS: {} | VEL: {})\n", debug_node.get::<Node3D>().get_position().round(), vel_mag).as_str());
    }

    for(children_of_debugger) in q_debugger_text_parents.iter() {
        for(&child) in children_of_debugger {
            if let Ok(mut label_node) = q_debugger_text_children.get_mut(child) {
                label_node.get::<Label>().set_text(&debug_text);
            };

        }
    }
}

#[derive(AssetCollection, Resource, Debug)]
#[derive(Default)]
pub struct DebugGameAssets {
    #[asset(path = "templates/DebuggerDrawer.tscn")]
    pub debugger_drawer_template: Handle<GodotResource>,
}

fn spawn_debug_tools(mut commands: Commands, debug_assets: Res<DebugGameAssets>) {
    commands
        .spawn_empty()
        .insert(GodotScene::from_handle(debug_assets.debugger_drawer_template.clone()))
        .insert(Transform::default())
        .insert((
            DebuggerMarker,
            DebuggerText,
        ));
}

#[derive(Component)]
pub struct DebuggerMarker;

#[derive(Component)]
pub struct DebuggerText;

#[derive(Component)]
pub struct DebugThisTransformMarker;