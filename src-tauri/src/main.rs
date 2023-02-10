#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lazy_static::lazy_static;
use scraper::{Html, Node};
use crate::song::{Song, Verse};
use regex::Regex;

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
    // let lyrics_selector = scraper::Selector::parse("div.Lyrics__Container-sc-1ynbvzw-6.YYrds").unwrap();
    // let lyrics = document.select(&lyrics_selector).map(|x| x.text().map(|a| a.to_owned()));
    // let mut children = document.select(&lyrics_selector).map(|x| x.children()).flatten();
    //
    // let mut verses: Vec<Verse> = Vec::new();
    // let mut current_verse: Vec<String> = Vec::new();
    // let mut current_num_elements_seen = 0;
    // for child in children {
    //     match child.value() {
    //         Node::Text(text) => {
    //             let text_string = text.to_string();
    //             println!("{:?}", text_string);
    //
    //             if text_string.starts_with("[") && text_string.ends_with("]") {
    //                 continue;
    //             }
    //
    //             if current_verse.len() > 0 && current_num_elements_seen > 1 {
    //                 verses.push(Verse::new(current_verse.clone()));
    //                 current_verse = Vec::new();
    //             }
    //
    //             current_verse.push(text.to_string());
    //             current_num_elements_seen = 0;
    //         },
    //         Node::Element(el) => {
    //             println!("{:?}", el.name);
    //
    //             current_num_elements_seen += 1;
    //         },
    //         _ => (),
    //     }
    // }

    let verses = parse_song_text(&document);

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

fn parse_song_text(document: &Html) -> Vec<Verse> {
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
        static ref remove_div_regex: Regex = Regex::new("<div>.*</div>").unwrap();
    }
    let verses_lines = verses_lines.map(|verse| verse.into_iter().map(|line| remove_div_regex.replace_all(&line, "").into_owned()));

    // Remove all lingering tags, e.g.: <i> and </i>
    // We want to keep the text in between
    lazy_static! {
        static ref remove_tag_regex: Regex = Regex::new("<.*?>").unwrap();
    }
    let verses_lines = verses_lines.map(|verse| verse.map(|line| remove_tag_regex.replace_all(&line, "").into_owned()));

    // Remove empty lines
    let verses_lines = verses_lines.map(|verse| verse.filter(|line| !line.is_empty()));

    let verses = verses_lines
        // .map(|verse| verse.map(|line| line.into_owned()))
        .map(|verse| Verse::new(verse.collect()))
        .collect();

    verses
}





fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_lyrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// https://genius.com/Justin-bieber-baby-lyrics

