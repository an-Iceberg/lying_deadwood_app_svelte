#![allow(clippy::needless_return)]

use std::{default, fs::create_dir, io::ErrorKind, path::Path, sync::Mutex};
use directories::UserDirs;
use polars::frame::DataFrame;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String
{
  return format!("Hello, {}! You've been greeted from Rust!", name);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run()
{
  tauri::Builder::default()
    .setup(|app|
    {
      app.manage(Mutex::new(LyingDeadwoodAppState::new()));
      return Ok(());
    })
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![increment, greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// https://v2.tauri.app/develop/state-management/

// Todo:
// Check, if files exist in /documents
// If yes, read them in and pass data to frontend

// Todo: make backend communicate with the frontend
// https://tauri.app/develop/calling-rust/
// https://tauri.app/develop/calling-frontend/

#[derive(Default)]
struct LyingDeadwoodAppState
{
  pub counter: u8, // dbg
  path_to_files: String,
  data: DataFrame
}

impl LyingDeadwoodAppState
{
  pub fn new() -> Self
  {
    // Todo: use safe Rust
    // Creates the directory ~/Documents/lyind_deadwood_app if it doesn't exist already
    let user_dirs = UserDirs::new().unwrap();
    let document_dir_path = user_dirs.document_dir().unwrap();
    let app_data_path = document_dir_path.join("lyind_deadwood_app");
    let app_data_path_str = app_data_path.to_str().unwrap().to_string();
    match create_dir(app_data_path)
    {
      Ok(_) => println!("successfully created {}", app_data_path_str),
      Err(error) => match error.kind()
      {
        ErrorKind::AlreadyExists => println!("{} exists already 👍", app_data_path_str),
        _ => println!("FS error: {error}"),
      },
    }

    let app = LyingDeadwoodAppState
    {
      path_to_files : app_data_path_str,
      ..Default::default()
    };

    return app;
  }

  pub fn inc(&mut self)
  {
    self.counter += 1;
  }
}

#[tauri::command]
fn increment(state: tauri::State<'_, Mutex<LyingDeadwoodAppState>>, amount: u32)
{
  let mut state = state.lock().unwrap();
  for _ in 0..amount { state.inc(); }
  println!("counter: {}", state.counter);
}

// #[tauri::command]
// async fn command_name(state: State<'_, Mutex<LyingDeadwoodAppState>>) -> Result<(), String>
// {
//   Ok(())
// }

// #[derive(Default)]
// struct MyState {
//   s: std::sync::Mutex<String>,
//   t: std::sync::Mutex<std::collections::HashMap<String, String>>,
// }
// // remember to call `.manage(MyState::default())`
// #[tauri::command]
// async fn command_name_2(state: tauri::State<'_, MyState>) -> Result<(), String> {
//   *state.s.lock().unwrap() = "new string".into();
//   state.t.lock().unwrap().insert("key".into(), "value".into());
//   Ok(())
// }
