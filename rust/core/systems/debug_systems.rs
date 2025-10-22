use crate::resources::input_resources::InputBuffer;
use crate::GameState;
use bevy::prelude::ops::sqrt;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use godot::classes::CharacterBody3D;
use godot::classes::Label;
use godot::prelude::*;
use godot_bevy::prelude::*;

pub struct DebuggingPlugin;
impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::InGame)
                .load_collection::<DebugGameAssets>(),
        )
        .init_resource::<InputBuffer>()
        .add_systems(
            Update,
            debugger_update_system.run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_debug_tools);
    }
}

#[main_thread_system]
pub fn debugger_update_system(
    q_debugger_text_parents: Query<&Children, With<DebuggerText>>,
    mut q_debugger_text_children: Query<&mut GodotNodeHandle, With<LabelMarker>>,
    mut q_debug_this_transform: Query<
        &mut GodotNodeHandle,
        (With<DebugThisTransformMarker>, Without<LabelMarker>),
    >,
    input_buffer: Res<InputBuffer>,
) {
    let mut debug_text: String = String::from("[DEBUG]\n");
    debug_text.push_str(format!("INPUT_MOVEMENTS: {}\n", input_buffer.get_movements()).as_str());
    debug_text.push_str(format!("MOUSE_BUFFER: {}\n", input_buffer.look_delta).as_str());
    for mut debug_node in q_debug_this_transform.iter_mut() {
        let vel = debug_node.get::<CharacterBody3D>().get_velocity();
        let vel_mag = sqrt(vel.x * vel.x + vel.z * vel.z).round();
        debug_text.push_str(
            format!(
                "(POS: {} | VEL: {})\n",
                debug_node.get::<Node3D>().get_position().round(),
                vel_mag
            )
            .as_str(),
        );
    }

    for children_of_debugger in q_debugger_text_parents.iter() {
        for &child in children_of_debugger {
            if let Ok(mut label_node) = q_debugger_text_children.get_mut(child) {
                label_node.get::<Label>().set_text(&debug_text);
            };
        }
    }
}

#[derive(AssetCollection, Resource, Debug, Default)]
pub struct DebugGameAssets {
    #[asset(path = "res://templates/DebuggerDrawer.tscn")]
    pub debugger_drawer_template: Handle<GodotResource>,
}

fn spawn_debug_tools(mut commands: Commands, debug_assets: Res<DebugGameAssets>) {
    commands
        .spawn(GodotScene::from_handle(
            debug_assets.debugger_drawer_template.clone(),
        ))
        .insert((DebuggerMarker, DebuggerText));
}

#[derive(Component)]
pub struct DebuggerMarker;

#[derive(Component)]
pub struct DebuggerText;

#[derive(Component)]
pub struct DebugThisTransformMarker;
