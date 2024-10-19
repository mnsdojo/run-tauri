use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use tauri::async_runtime::spawn;
use tauri::Emitter;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_cmd(cmd: &str, app_handle: tauri::AppHandle) -> Result<(), String> {
    // split the stuff
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    let program = parts[0];
    let args = &parts[1..];

    // Create command with piped stdout
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let app_handle_clone = app_handle.clone();

    // Handle stdout
    spawn(async move {
        let reader = BufReader::new(stdout);
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let _ = app_handle_clone.emit("cmd-output", format!("stdout: {}", line));
            }
        });
    });

    // Handle stderr
    let app_handle_clone = app_handle.clone();
    spawn(async move {
        let reader = BufReader::new(stderr);
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                let _ = app_handle_clone.emit("cmd-output", format!("stderr: {}", line));
            }
        });
    });

    // Wait for the command to complete
    let status = child.wait().map_err(|e| e.to_string())?;

    app_handle
        .emit("cmd-finished", status.success())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, run_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
