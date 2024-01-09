// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use tauri::{AppHandle, WindowBuilder, WindowUrl};
use std::{path::{Path, self}, fs::{self, OpenOptions, File}, collections::HashMap, io::{Write, self, Read}};


#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    editor: String,
    path: String,
    description: String,
}

fn get_state() -> Result<HashMap<String, Project>, io::Error> {
    let path = "settings.json";

    // Check if the file exists
    if !Path::new(path).exists() {
        // Return an empty state if the file doesn't exist
        return Ok(HashMap::new());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize JSON to HashMap<String, Project>
    let state: HashMap<String, Project> = serde_json::from_str(&contents)?;

    Ok(state)
}

fn update_data(key: &str, value: Project, state: &mut HashMap<String, Project>) {
    state.insert(key.to_string(), value);
    println!("state updated");
}

fn save_state(state: &HashMap<String, Project>) -> Result<(), io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("settings.json")?;

    // Serialize the HashMap to JSON
    let settings = serde_json::to_string_pretty(state)?;

    // Write JSON to the file
    let mut file = file;
    file.write_all(settings.as_bytes())?;

    Ok(())
}

#[tauri::command]
async fn getState() -> Result<HashMap<String, Project>, String> {
    match get_state() {
        Ok(state) => Ok(state),
        Err(err) => Err(format!("Error reading state from file: {:?}", err)),
    }
}

#[tauri::command]
async fn updateData(key: String, value: Project) -> Result<(), String> {
    let mut state = match get_state() {
        Ok(state) => state,
        Err(err) => return Err(format!("Error reading state from file: {:?}", err)),
    };

    update_data(&key, value, &mut state);

    if let Err(err) = save_state(&state) {
        Err(format!("Error saving state: {:?}", err))
    } else {
        Ok(())
    }
}

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
    path::Path::new(&path).exists()
}

#[tauri::command]
fn create_project(name: String, path: String, editor: String, description: String) -> bool {
    std::fs::create_dir(path).unwrap();
    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_new_project_window, check_path, create_project, getState, updateData])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
