// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize, de};
use tauri::{AppHandle, WindowBuilder, WindowUrl};
use std::{path::{Path, self}, fs::{self, OpenOptions, File}, collections::HashMap, io::{Write, self, Read}, fmt::format};
use chrono::{Datelike, Timelike, Utc, Local};


#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    editor: String,
    path: String,
    description: String,
    date_created: String,
    date_modified: String,
    time_created: String,
    time_modified: String,
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
    println!("updateData()");

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

fn create_dir(dir_path: &str) -> Result<(), std::io::Error> {
    if !fs::metadata(dir_path).is_ok() {
        fs::create_dir(dir_path)?;
        println!("Directory '{}' created successfully", dir_path);
    } else {
        println!("Directory '{}' already exists", dir_path);
    }
    Ok(())
}


fn create_project(name: String, path: String, description: String) -> Result<(), io::Error> {
    match create_dir(&path) {
        Ok(_) => println!("Operation completed successfully"),
        Err(err) => eprintln!("Error: {}", err),
    }

    
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path.clone() + "\\README.md")?;

    let mut file = file;
    file.write_all(format!("# {}\n{}", name, description).as_bytes())?;

    Ok(())
}

#[tauri::command]
fn createProject(name: String, path: String, description: String) -> bool {
    if let Err(_err) = create_project(name, path, description) {
        false
    } else {
        true
    }
}

#[tauri::command]
fn get_time() -> String {
    let now = Local::now();

    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

#[tauri::command]
fn get_date() -> String {
    let now = Local::now();

    format!("{:04}:{:02}:{:02}", now.year(), now.month(), now.day())
}

fn main() {
    let now = Local::now();
    println!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());

    

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_new_project_window, check_path, createProject, getState, updateData, get_date, get_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
