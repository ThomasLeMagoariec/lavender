// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::de::value::Error;
use tauri::{AppHandle, WindowBuilder, WindowUrl};
use std::path::{Path, self};


#[tauri::command]
async fn open_new_project_window(app: AppHandle) -> Result<(), String> {
    let result = WindowBuilder::new(&app, "settings", WindowUrl::App("newproject/newproject.html".into()))
        .fullscreen(false)
        .resizable(true)
        .title("New Project")
        .center()
        .build();
    match result {
        Ok(_) => {
            println!("Window Created Successfully!");
            Ok(())
        }
        Err(err) => {
            println!("Failed to Create Window {}", err);
            Err("Failed to create Window".to_string())
        }
    }
}



#[tauri::command]
fn check_path(path: String) -> bool {
    !path::Path::new(&path).exists()
}

#[tauri::command]
fn create_project(name: String, path: String, editor: String, description: String) -> bool {
    std::fs::create_dir(path).unwrap();
    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_new_project_window, check_path, create_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
