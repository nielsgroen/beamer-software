use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongList {
    pub songs: Vec<SongSlot>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SongSlotType {
    Empty,
    Song(Song),
    // Can add more: Like a picture
}

#[derive(Serialize, Deserialize, Debug , Clone)]
pub struct SongSlot {
    pub id: u64,
    pub slot: SongSlotType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Song {
    pub title: String,
    pub author: String,
    pub verses: Vec<Verse>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Verse {
    pub lines: Vec<String>,
}

impl Song {
    pub fn new(title: &str, author: &str, verses: Vec<Verse>) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            verses,
        }
    }
}

impl Verse {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines
        }
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
