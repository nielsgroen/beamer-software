use std::cmp::{max, min};
use std::vec;
use serde::Serialize;
use crate::song::{Song, SongList, SongSlotType, Verse};

/// For tracking which verse to currently show

#[derive(Clone, Debug, Serialize)]
pub struct DisplaySelection {
    slot_id: u64,
    slot_position: usize,
    verse_num: usize,
    song: Song,
}

impl DisplaySelection {
    pub fn new(song_list: &SongList, song_position: usize, verse_num: Option<usize>) -> Self {
        let verse_num = verse_num.unwrap_or(0);

        let song_slot = &song_list.songs[song_position];
        let slot_id = song_slot.id;

        let song = Self::unwrap_or_song(&song_slot.slot);

        if song.verses.get(verse_num).is_none() {
            panic!("verse_num is {}, but song is of length {}", verse_num, song.verses.len());
        }

        Self {
            slot_id,
            slot_position: song_position,
            verse_num,
            song,
        }
    }

    pub fn current_verse(&self) -> &Verse {
        &self.song.verses[self.verse_num]
    }

    pub fn previous(&mut self, song_list: &SongList) {
        if self.verse_num > 0 {
            self.verse_num -= 1;
        } else {
            let current_position = song_list.songs.iter().position(|x| x.id == self.slot_id);

            // saturating sub: prevent underflow
            let new_position = current_position.map(|x| x.saturating_sub(1)).unwrap_or(self.slot_position);

            if new_position >= 0 && new_position < song_list.songs.len() {
                let next_song = song_list.songs[new_position].clone();
                let verse_num = match next_song.slot {
                    SongSlotType::Empty => 0,
                    SongSlotType::Song(ref song) => song.num_verses() - 1,
                };

                self.slot_id = next_song.id;
                self.slot_position = new_position;
                self.verse_num = verse_num;
                self.song = Self::unwrap_or_song(&next_song.slot);

            } else {
                if let Some(next_song) = song_list.songs.last() {
                    self.slot_id = next_song.id;
                    self.slot_position = song_list.songs.len() - 1;
                    self.verse_num = 0;
                    self.song = Self::unwrap_or_song(&next_song.slot);

                } else {
                    self.slot_id = 0;
                    self.slot_position = 0;
                    self.verse_num = 0;
                    self.song = Self::unwrap_or_song(&SongSlotType::Empty);
                }
            }
        }
    }

    pub fn next(&mut self, song_list: &SongList) {
        if self.verse_num < self.song.verses.len() - 1 {
            // Next verse in same song available: Go there
            self.verse_num += 1;
        } else {
            // Go to next song
            let current_position = song_list.songs.iter().position(|x| x.id == self.slot_id);

            // If song removed: Go to song that is now at that position
            let song_list_len = song_list.songs.len();
            let new_position= current_position.map(|x| x + 1).unwrap_or(self.slot_position);

            if new_position < song_list.songs.len() {
                // Go to next if not at end
                let next_song = song_list.songs[new_position].clone();

                self.slot_id = next_song.id;
                self.slot_position = new_position;
                self.verse_num = 0;
                self.song = Self::unwrap_or_song(&next_song.slot);
            } else {
                // Or if index more than songlist length: Go to last song
                // If Songlist empty: Go to empty song slot with id 0

                if let Some(next_song) = song_list.songs.last() {
                    self.slot_id = next_song.id;
                    self.slot_position = song_list.songs.len() - 1;
                    self.verse_num = next_song.slot.num_verses().saturating_sub(1);
                    self.song = Self::unwrap_or_song(&next_song.slot);

                } else {
                    self.slot_id = 0;
                    self.slot_position = 0;
                    self.verse_num = 0;
                    self.song = Self::unwrap_or_song(&SongSlotType::Empty);
                }
            }
        }
    }

    fn unwrap_or_song(slot_type: &SongSlotType) -> Song {
        match slot_type {
            SongSlotType::Empty => Song::new("Empty Panel", "None", vec![Verse::new(vec![])]),
            SongSlotType::Song(song) => song.clone(),
        }
    }
}
