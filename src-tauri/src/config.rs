use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::Manager;
use crate::ProgramState;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub config_path: PathBuf,
    pub genius_api_token: Option<String>,
    pub font_size: String,
}


#[tauri::command]
pub async fn get_genius_token(
    program_state: tauri::State<'_, ProgramState>,
) -> Result<String, ()> {
    let config = program_state.config.read().await;

    Ok((*config).genius_api_token.clone().unwrap_or("".to_string()))
}

#[tauri::command]
pub async fn get_font_size(
    program_state: tauri::State<'_, ProgramState>,
) -> Result<String, ()> {
    let config = program_state.config.read().await;

    Ok((*config).font_size.clone())
}

#[tauri::command]
pub async fn set_genius_token(
    new_token: String,
    program_state: tauri::State<'_, ProgramState>,
) -> Result<(), ()> {
    let mut config = program_state.config.write().await;

    config.genius_api_token = Some(new_token);

    Ok(())
}

#[tauri::command]
pub async fn set_font_size(
    new_font_size: String,
    program_state: tauri::State<'_, ProgramState>,
    app_handle: tauri::AppHandle,
) -> Result<(), ()> {
    let mut config = program_state.config.write().await;
    config.font_size = new_font_size.clone();

    app_handle.emit_to("presentation", "update-font-size", new_font_size).expect("could not emit update-font-size");
    Ok(())
}

#[tauri::command]
pub async fn save_config(
    program_state: tauri::State<'_, ProgramState>,
) -> Result<(), String> {
    let config = program_state.config.read().await;

    let json = serde_json::to_string(&*config).map_err(|x| "Unable to parse JSON".to_string())?;
    // fs::write(config.config_path.clone(), json).map_err(|x| "Unable to write to config".to_string())?;
    let new_path = config.config_path.clone();
    fs::create_dir_all(config.config_path.parent().unwrap()).unwrap();
    fs::write(new_path, json).unwrap();

    Ok(())
}

