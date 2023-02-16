#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lazy_static::lazy_static;
use scraper::{Html, Node};
use crate::song::{Song, SongList, SongSlot, SongSlotType, Verse};
use regex::Regex;
use anyhow::Result;
use tauri::async_runtime::RwLock;

mod song;

/// IMPORTANT ALWAYS ACQUIRE LOCKS IN ORDER LISTED
pub struct ProgramState {
    pub song_list: RwLock<SongList>,
    pub new_song_id: RwLock<u64>,
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
) -> Result<SongList, ()> {

    let new_song = get_lyrics(
        author,
        title,
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
async fn get_lyrics(author: &str, title: &str) -> Result<Song, ()> {
    let response = reqwest::get(
        "https://genius.com/Justin-bieber-baby-lyrics",
    )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = scraper::Html::parse_document(&response);

    let verses = parse_song_text(&document, true);

    Ok(Song {
        title: "Baby".to_string(),
        author: "Justin Bieber".to_string(),
        verses,
    })
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
        .manage(ProgramState {
            song_list: RwLock::new(SongList {
                songs: vec![
                    SongSlot {
                        id: 0,
                        slot: SongSlotType::Empty,
                    },
                ],
            }),
            new_song_id: RwLock::new(1),
        })
        .invoke_handler(tauri::generate_handler![get_songs, get_lyrics, add_searched_song, update_song_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
