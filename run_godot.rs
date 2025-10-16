use std::env;
use std::process::{Command, Stdio};

fn main() {
    let run_dir = format!("{}/godot/", env!("CARGO_MANIFEST_DIR"));
    let godot_global_binary = which::which("godot").map(|x| Some(x.to_string_lossy().to_string())).unwrap_or_else(|_| None);

    let godot_local_binary = option_env!("GODOT").map(|x| x.to_string());

    let mut godot_binary_optional: Option<String> = None;
    if godot_global_binary.is_some() {
        godot_binary_optional = Some(godot_global_binary.unwrap());
    }
    else if godot_local_binary.is_some() {
        godot_binary_optional = godot_local_binary;
    }

    let godot_binary = godot_binary_optional.expect("`GODOT` cli command not configured. Please set the `GODOT` environmental variable.");
    
    Command::new(&godot_binary)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(&run_dir)
        .output()
        .unwrap_or_else(|_| panic!("tried running `{}` to edit this project and failed.", &godot_binary));
}