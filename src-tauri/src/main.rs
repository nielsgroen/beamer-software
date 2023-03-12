#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::error::Error;
use std::fs;
use lazy_static::lazy_static;
use scraper::{Html, Node};
use crate::song::{Song, SongList, SongSlot, SongSlotType, Verse};
use regex::Regex;
use anyhow::Result;
use tauri::async_runtime::RwLock;
use tauri::Manager;
use tower_service::Service;

mod song;
mod config;
mod display_selection;

use config::ProgramConfig;
use crate::display_selection::DisplaySelection;

const SEARCH_URL: &str = "https://api.genius.com/search";


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
async fn add_searched_song(
    author: &str,
    title: &str,
    program_state: tauri::State<'_, ProgramState>,
) -> Result<SongList, String> {
    let genius_token = program_state.config.read().await.genius_api_token
        .clone()
        .ok_or("No Genius API token".to_string())?;

    let (song_url, actual_author, actual_title) = find_song_details(author, title, &genius_token).await?;

    let new_song = get_lyrics(
        &song_url,
        &actual_author,
        &actual_title
    ).await?;

    let mut song_list = program_state.song_list.write().await;
    let mut new_song_id = program_state.new_song_id.write().await;

    song_list.songs.push(
        SongSlot {
            id: *new_song_id,
            slot: SongSlotType::Song(new_song),
        }
    );
    *new_song_id += 1;

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
async fn get_lyrics(song_url: &str, author: &str, title: &str) -> Result<Song, String> {
    println!("{:?}", song_url);
    let song_url = reqwest::Url::parse(song_url).map_err(|x| {
        println!("parse error {:?}", x);
        "Invalid URL".to_string()
    })?;
    let response = reqwest::get(song_url.to_string())
        .await
        .map_err(|x| {
            println!("{:?}", x);
            "Unable to load song page".to_string()
        })?
        .text()
        .await
        .map_err(|_| "Unable to extract text from song page".to_string())?;

    let document = scraper::Html::parse_document(&response);

    let verses = parse_song_text(&document, true);

    Ok(Song {
        title: title.to_string(),
        author: author.to_string(),
        verses,
    })
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

async fn find_song_details(
    author: &str,
    title: &str,
    genius_token: &str,
) -> Result<(String, String, String), String> { // Ok(url, actual author, actual title)
    let search_string = format!("{} {}", author, title);

    let mut client = reqwest::Client::new();
    let request = client.get(SEARCH_URL)
        .header(reqwest::header::ACCEPT, "application/json")
        .bearer_auth(genius_token)
        .query(&[("q", search_string)])
        .build()
        .map_err(|x| x.to_string())?;

    let response = client.call(request).await.map_err(|x| x.to_string())?;
    println!("response: {:?}", response);

    let response_json: serde_json::Value = response.json().await.map_err(|x| x.to_string())?;
    println!("response json: {:?}", response_json);

    let mut url: String = "".to_string();
    let mut actual_author: String = "".to_string();
    let mut actual_title: String = "".to_string();
    if let Some(hits) = response_json["response"]["hits"].as_array() {
        for hit in hits {
            if hit["type"] == "song" {
                match hit["result"]["url"].clone() {
                    serde_json::Value::String(x) => url = x,
                    _ => return Err("Failed to parse url to string".to_string()),
                };

                match hit["result"]["artist_names"].clone() {
                    serde_json::Value::String(x) => actual_author = x,
                    _ => return Err("Failed to parse artist_names to string".to_string()),
                };

                match hit["result"]["title"].clone() {
                    serde_json::Value::String(x) => actual_title = x,
                    _ => return Err("Failed to parse title to string".to_string()),
                };
            }
            break;
        }

        Ok((url, actual_author, actual_title))
    } else {
        Err("Song not found".to_string())
    }
}


fn parse_song_text(document: &Html, remove_block_quotes: bool) -> Vec<Verse> {
    let lyrics_selector = scraper::Selector::parse("div.Lyrics__Container-sc-1ynbvzw-6.YYrds").unwrap();

    let html_sections = document.select(&lyrics_selector)
        .map(|x| x.inner_html());

    let verse_sections = html_sections
        .map(|x| {
            let lines = x.split("<br><br>");
            lines
                .map(|line| line.to_owned())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let verses_lines = verse_sections.into_iter()
        .map(|x| {
            let lines = x.split("<br>");
            lines
                .map(|line| line.to_owned())
                .collect::<Vec<_>>()
        });

    // Remove all divs with contents: usually ads
    lazy_static! {
        static ref REMOVE_DIV_REGEX: Regex = Regex::new("<div>.*</div>").unwrap();
    }
    let verses_lines = verses_lines.map(|verse| verse.into_iter().map(|line| REMOVE_DIV_REGEX.replace_all(&line, "").into_owned()));

    // Remove all lingering tags, e.g.: <i> and </i>
    // We want to keep the text in between
    lazy_static! {
        static ref REMOVE_TAG_REGEX: Regex = Regex::new("<.*?>").unwrap();
    }
    let verses_lines = verses_lines.map(|verse| verse.map(|line| REMOVE_TAG_REGEX.replace_all(&line, "").into_owned()));

    // Remove all Blockquotes if that option is enabled, while making sure types stay consistent
    let new_verses_lines;
    if remove_block_quotes {
        lazy_static! {
            static ref REMOVE_BLOCKQUOTES_REGEX: Regex = Regex::new(r#"\[.*?\]"#).unwrap();
        }
        new_verses_lines = verses_lines
            .map(|verse| {
                let a = verse.map(|line| REMOVE_BLOCKQUOTES_REGEX.replace_all(&line, "").into_owned());
                a.collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into_iter();
    } else {
        new_verses_lines = verses_lines
            .map(|verse| verse.collect())
            .collect::<Vec<_>>()
            .into_iter();
    }

    // Remove empty lines
    let verses_lines = new_verses_lines.map(|verse| verse.into_iter().filter(|line| !line.is_empty()));

    let verses = verses_lines
        // .map(|verse| verse.map(|line| line.into_owned()))
        .map(|verse| Verse::new(verse.collect()))
        .filter(|verse| !verse.is_empty())
        .collect();

    verses
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
