#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use scraper::Node;
use crate::song::{Song, Verse};

mod song;

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn get_lyrics(author: &str, title: &str) -> Song {
    let response = reqwest::blocking::get(
        "https://genius.com/Justin-bieber-baby-lyrics",
    )
        .unwrap()
        .text()
        .unwrap();
    let document = scraper::Html::parse_document(&response);
    let lyrics_selector = scraper::Selector::parse("div.Lyrics__Container-sc-1ynbvzw-6.YYrds").unwrap();
    // let lyrics = document.select(&lyrics_selector).map(|x| x.text().map(|a| a.to_owned()));
    let mut children = document.select(&lyrics_selector).map(|x| x.children()).flatten();

    let mut verses: Vec<Verse> = Vec::new();
    let mut current_verse: Vec<String> = Vec::new();
    let mut current_num_elements_seen = 0;
    for child in children {
        match child.value() {
            Node::Text(text) => {
                let text_string = text.to_string();
                println!("{:?}", text_string);

                if text_string.starts_with("[") && text_string.ends_with("]") {
                    continue;
                }

                if current_verse.len() > 0 && current_num_elements_seen > 1 {
                    verses.push(Verse::new(current_verse.clone()));
                    current_verse = Vec::new();
                }

                current_verse.push(text.to_string());
                current_num_elements_seen = 0;
            },
            Node::Element(el) => {
                println!("{:?}", el.name);
                current_num_elements_seen += 1;
            },
            _ => (),
        }
    }

    Song {
        title: "Baby".to_string(),
        author: "Justin Bieber".to_string(),
        verses,
    }
    // for verse in verses {
    //     println!("{:?}", verse);
    // }

    // lyrics.collect::<Vec<_>>().join("\n")
    // verses.flatten().collect()
    // vec!["Henkie".to_string()]
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_lyrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// https://genius.com/Justin-bieber-baby-lyrics

