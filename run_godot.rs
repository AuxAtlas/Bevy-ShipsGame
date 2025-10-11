use std::process::{Command, Stdio};

fn main() {
    let run_dir = format!("{}/../", env!("CARGO_MANIFEST_DIR"));
    let godot_path = option_env!("GODOT").expect("Please set GODOT env var.");
    Command::new(godot_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(&run_dir)
        .output()
        .unwrap_or_else(|_| {
            panic!("tried running `{}` to edit this project and failed.", godot_path)
        });
}