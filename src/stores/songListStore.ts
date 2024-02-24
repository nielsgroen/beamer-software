import {defineStore} from "pinia";
import {invoke} from "@tauri-apps/api";

export const useSongListStore = defineStore('songList', {
    state: () => ({
        songs: [],
    }),
    actions: {
        async updateBackend(newSongs: any) {
            await invoke("update_song_list", {
                newSongList: {
                    songs: newSongs,
                },
            });
            this.songs = newSongs;
        },
        async removeFirst() {
            const newSongs = this.songs;
            newSongs.shift();

            await this.updateBackend(newSongs);
        },
        async removeById(song_id: any) {
            console.log("song_id", song_id);
            let newSongs: any = this.songs;
            newSongs = newSongs.filter((song: any) => song.id != song_id)

            await this.updateBackend(newSongs);
        },
        async addSearchedSong(author: string, title: string) {
            const result: any = await invoke('add_searched_song', { author, title });
            this.songs = result.songs;
        },
        async addSong(songAddition: any) {
            const newSongList: any = await invoke("add_song", songAddition);
            this.songs = newSongList.songs;
        }
    }
})
