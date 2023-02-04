#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn get_lyrics(author: &str, title: &str) -> Vec<String> {
    let response = reqwest::blocking::get(
        "https://genius.com/Justin-bieber-baby-lyrics",
    )
        .unwrap()
        .text()
        .unwrap();
    let document = scraper::Html::parse_document(&response);
    let lyrics_selector = scraper::Selector::parse("div.Lyrics__Container-sc-1ynbvzw-6.YYrds").unwrap();
    let lyrics = document.select(&lyrics_selector).map(|x| x.text().map(|a| a.to_owned()));

    // lyrics.collect::<Vec<_>>().join("\n")
    lyrics.flatten().collect()
    // "Henkie".to_string()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_lyrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// https://genius.com/Justin-bieber-baby-lyrics

