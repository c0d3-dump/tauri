use std::{fs::File, sync::Mutex};

use tauri::{path::BaseDirectory, AppHandle, Manager};

use crate::AppData;

#[tauri::command]
pub fn result(user_name: &str) -> Result<String, String> {
    println!("{}", user_name);
    if user_name == "b2b" {
        Ok(format!("Hello there {}!", user_name))
    } else {
        Err("Invalid name".into())
    }
}

#[tauri::command]
pub fn option(handle: AppHandle, user_name: &str) -> Option<String> {
    println!("{}", user_name);

    let state = handle.state::<Mutex<AppData>>();
    dbg!(state);

    if user_name == "b2b" {
        Some(format!("Hello there {}!", user_name))
    } else {
        None
    }
}

#[tauri::command]
pub async fn async_func(handle: AppHandle) -> Result<String, ()> {
    let resource_path = handle
        .path()
        .resolve("lang/en.json", BaseDirectory::Resource)
        .unwrap();

    let file = File::open(&resource_path).unwrap();
    let lang_en: serde_json::Value = serde_json::from_reader(file).unwrap();

    dbg!(lang_en);

    let state = handle.state::<Mutex<AppData>>();

    let mut state = state.lock().unwrap();
    state.counter = 10;

    Ok("Here you go".into())
}
