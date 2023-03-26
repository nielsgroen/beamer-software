#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use std::fs;
use lazy_static::lazy_static;
use scraper::{Html, Node};
use song::{Song, SongList, SongSlot, SongSlotType, Verse};
use regex::Regex;
use anyhow::Result;
use tauri::async_runtime::RwLock;
use tauri::Manager;
use tower_service::Service;
use querying::{add_searched_song, get_lyrics};

mod song;
mod config;
mod display_selection;
mod querying;

use config::ProgramConfig;
use crate::display_selection::DisplaySelection;



/// IMPORTANT: ALWAYS ACQUIRE LOCKS IN ORDER LISTED
pub struct ProgramState {
    pub config: RwLock<ProgramConfig>,
    pub song_list: RwLock<SongList>,
    pub new_song_id: RwLock<u64>,
    pub currently_selected: RwLock<DisplaySelection>,
    //     ... e.g. currently showing slide
}

#[tauri::command]
async fn get_songs(
    program_state: tauri::State<'_, ProgramState>,
) -> Result<SongList, ()> {
    let song_list  = program_state.song_list.read().await;
    Ok((*song_list).clone())
}


#[tauri::command]
async fn update_song_list(
    new_song_list: SongList,
    program_state: tauri::State<'_, ProgramState>,
) -> Result<(), ()> {
    let mut song_list = program_state.song_list.write().await;
    *song_list = new_song_list;

    Ok(())
}


#[tauri::command]
async fn get_genius_token(
    program_state: tauri::State<'_, ProgramState>,
) -> Result<String, ()> {
    let config = program_state.config.read().await;

    Ok((*config).genius_api_token.clone().unwrap_or("".to_string()))
}

#[tauri::command]
async fn set_genius_token(
    new_token: String,
    program_state: tauri::State<'_, ProgramState>,
) -> Result<(), ()> {
    let mut config = program_state.config.write().await;

    config.genius_api_token = Some(new_token);

    Ok(())
}

#[tauri::command]
async fn save_config(
    program_state: tauri::State<'_, ProgramState>,
) -> Result<(), String> {
    let config = program_state.config.read().await;

    let json = serde_json::to_string(&*config).map_err(|x| "Unable to parse JSON".to_string())?;
    // fs::write(config.config_path.clone(), json).map_err(|x| "Unable to write to config".to_string())?;
    let new_path = config.config_path.clone();
    println!("config path {:?}", new_path);
    fs::create_dir_all(config.config_path.parent().unwrap()).unwrap();
    fs::write(new_path, json).unwrap();

    Ok(())
}

#[tauri::command]
async fn next_verse(
    program_state: tauri::State<'_, ProgramState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use tauri::Manager;

    let song_list = program_state.song_list.read().await;
    let mut selection = program_state.currently_selected.write().await;

    selection.next(&song_list);
    app_handle.emit_to("presentation", "update-verse", selection.current_verse()).expect("could not emit update-verse");

    Ok(())
}

#[tauri::command]
async fn previous_verse(
    program_state: tauri::State<'_, ProgramState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    use tauri::Manager;

    let song_list = program_state.song_list.read().await;
    let mut selection = program_state.currently_selected.write().await;

    selection.previous(&song_list);
    app_handle.emit_to("presentation", "update-verse", selection.current_verse()).expect("could not emit update-verse");

    Ok(())
}


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut config_path = app.path_resolver()
                .app_config_dir()
                .expect("No config directory found");
            config_path.push("config.json");

            let mut config: ProgramConfig = fs::read_to_string(&config_path).ok().and_then(|x| serde_json::from_str(&x).ok()).unwrap_or_default();
            config.config_path = config_path;

            let song_list = SongList {
                songs: vec![
                    SongSlot {
                        id: 0,
                        slot: SongSlotType::Empty,
                    },
                ],
            };
            let display_selection = DisplaySelection::new(&song_list, 0, None);
            (*app).manage(ProgramState {
                config: RwLock::new(config),
                song_list: RwLock::new(song_list),
                new_song_id: RwLock::new(1),
                currently_selected: RwLock::new(display_selection),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_songs,
            get_lyrics,
            add_searched_song,
            update_song_list,
            get_genius_token,
            set_genius_token,
            save_config,
            next_verse,
            previous_verse,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
