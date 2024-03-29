use std::vec;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use crate::{add_song_to_state, ProgramState};
use crate::song::{Song, SongList, SongSlot, SongSlotType, Verse};
use tower_service::Service;
use crate::display_selection::DisplaySelection;

const SEARCH_URL: &str = "https://api.genius.com/search";

#[tauri::command]
pub async fn add_searched_song(
    author: &str,
    title: &str,
    program_state: tauri::State<'_, ProgramState>,
    app_handle: tauri::AppHandle,
) -> Result<SongList, String> {
    use tauri::Manager;

    let genius_token = read_genius_token(&program_state).await?;

    let (song_url, actual_author, actual_title) = find_song_details(author, title, &genius_token).await?;

    let mut new_song = get_lyrics(
        &song_url,
        &actual_author,
        &actual_title
    ).await?;

    if new_song.verses.len() == 0 {
        new_song.verses = vec![Verse::default()];
    }

    add_song_to_state(
        new_song,
        &program_state,
    ).await;

    let mut song_list = program_state.song_list.write().await;

    let display_selection = program_state.currently_selected.read().await;
    let display_selection = display_selection.clone();
    let mut next_display_selection = display_selection.clone();
    next_display_selection.next(&song_list);
    app_handle.emit_to("main", "update-display-selection", (display_selection, next_display_selection)).expect("could not emit update-display-selection");

    Ok((*song_list).clone())
}

#[tauri::command]
pub async fn get_lyrics(song_url: &str, author: &str, title: &str) -> Result<Song, String> {
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
    // Lyrics__Container([a-zA-Z]|-|\d)+
    // Remove all divs with contents: usually ads
    lazy_static! {
        static ref FIND_LYRICS_CONTAINER_CLASS_REGEX: Regex = Regex::new(r#"Lyrics__Container([a-zA-Z]|-|\d)+"#).unwrap();
    }

    let document_string = document.html();
    let mut container_css_class = None;
    if let Some(capture) = FIND_LYRICS_CONTAINER_CLASS_REGEX.captures(&document_string) {
        if let Some(capture_match) = capture.get(0) {
            container_css_class = Some(capture_match.as_str().to_string());
        }
    }

    if container_css_class.is_none() {
        return vec![Verse::default()];
    }
    let container_css_class = container_css_class.unwrap();
    let lyrics_selector = scraper::Selector::parse(&format!(".{container_css_class}")).unwrap();

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

pub async fn read_genius_token(
    program_state: &tauri::State<'_, ProgramState>,
) -> Result<String, String> {
    program_state.config.read().await.genius_api_token
        .clone()
        .ok_or("No Genius API token".to_string())
}

