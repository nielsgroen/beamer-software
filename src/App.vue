<script lang="ts">
import {onMounted, ref, watch} from "vue";
// import SongEditor from "./components/SongEditor.vue";
import {invoke} from "@tauri-apps/api";
import SongList from "./components/SongList.vue";
import {useToast} from "primevue/usetoast";
import Toast from 'primevue/toast';
import {register} from "@tauri-apps/api/globalShortcut";
import SongEditor from "./components/SongEditor.vue";


export default {
  components: {SongEditor, SongList},
  setup() {
    const toast = useToast();

    const songList = ref([]);
    const selectedSong = ref([]);
    const geniusToken = ref("");

    const searchTitle = ref("");
    const searchAuthor = ref("");

    const songAddition = ref({
      title: "",
      author: "",
      songText: "",
    });

    onMounted(async () => {
      try {
        await register('right', () => {
          nextVerse();
        });
        await register('left', () => {
          previousVerse();
        });
      } catch (error) {
        console.error(error);
      }

      const result: any = await invoke("get_songs", {});
      songList.value = result.songs;
      console.log("songList mounted", songList);

      const newToken: string = await invoke("get_genius_token", {});
      geniusToken.value = newToken;
    })

    async function removeFirst() {
      const newSongList = songList.value;
      newSongList.shift();
      await processClientSongListUpdate(newSongList);
    }

    async function addSearchedSong(author: String, title: String) {
      try {
        toast.add({
          severity: "info",
          summary: "Searching",
          detail: `Searching for ${author} - ${title}.`,
          life: 3000,
        });
        const result: any = await invoke('add_searched_song', { author, title });
        songList.value = result.songs;
        toast.add({
          severity: "success",
          summary: "Song Added",
          detail: `${author} - ${title} was added.`,
          life: 3000,
        });
        console.log("Toast called");
      } catch (error) {
        console.error(error);
        console.log("error called");
        toast.add({
          severity: "error",
          summary: "Failed to load song",
          detail: error,
          life: 3000,
        })
      }
    }

    async function processClientSongListUpdate(newSongList: any) {
      console.log("process", newSongList);

      try {
        const result = await invoke("update_song_list", {
          newSongList: {
            songs: newSongList
          }
        });
        console.log("result", result);

        songList.value = newSongList;
      } catch (error) {
        toast.add({
          severity: "error",
          summary: "update failed",
          detail: "Failed to update the song list.",
          life: 3000,
        })
      }
    }

    async function saveToken() {
      await invoke("set_genius_token", { newToken: geniusToken.value });
    }

    async function saveConfig() {
      try {
        await invoke("save_config", {});
      } catch (error) {
        console.error(error);
      }
    }

    async function nextVerse() {
      try {
        await invoke("next_verse", {});
      } catch (error) {
        console.error(error);
      }
    }

    async function previousVerse() {
      try {
        await invoke("previous_verse", {});
      } catch (error) {
        console.error(error);
      }
    }

    async function addSong() {
      try {
        toast.add({
          severity: "info",
          summary: "Adding Song",
          detail: "The song is being added.",
          life: 3000,
        });
        const newSongList: any = await invoke("add_song", songAddition.value);
        songList.value = newSongList.songs;
        toast.add({
          severity: "success",
          summary: "Addition successful",
          detail: "Added song to song list.",
          life: 3000,
        });
      } catch (error) {
        toast.add({
          severity: "error",
          summary: "Addition failed",
          detail: "Failed to update the song list.",
          life: 3000,
        });
      }
    }


    watch(songList, (currentValue, oldValue) => {
      console.log("old val", oldValue);
      console.log("new val", currentValue);
    });

    watch(selectedSong, (currentValue, oldValue) => {
      console.log("old selected", oldValue);
      console.log("new selected", currentValue);
    })

    return {
      songList,
      selectedSong,
      geniusToken,
      searchTitle,
      searchAuthor,
      songAddition,
      processClientSongListUpdate,
      onMounted,
      removeFirst,
      addSearchedSong,
      saveToken,
      saveConfig,
      nextVerse,
      previousVerse,
      addSong,
    }
  }
}
</script>

<template>
  <div class="surface-ground px-4 py-8 md:px-6 lg:px-8 max-vh">
    <div class="text-900 font-bold text-6xl mb-4 text-center">Beamer Software</div>
    <div class="grid">
      <div class="col-12 lg:col-8">
        <div class="p-3 h-full">
          <div class="shadow-2 p-3 h-full flex flex-column surface-card">
            <SongList :model-value="songList" @update:model-value="processClientSongListUpdate($event)" @update:song-selection="selectedSong = $event" />
            <div class="grid">
              <div class="col-6">
                <span class="p-float-label">
                  <InputText id="search-author" type="text" v-model="searchAuthor" />
                  <label for="search-author">Search Author</label>
                </span>
              </div>
              <div class="col-6">
                <span class="p-float-label">
                  <InputText id="search-title" type="text" v-model="searchTitle" />
                  <label for="search-title">Search Title</label>
                </span>
              </div>
              <div class="col-6">
                <Button label="Remove First Song" class="p-button-danger" @click="removeFirst" />
              </div>
              <div class="col-6">
                <Button label="Add Searched Song" class="p-button-success" @click="addSearchedSong(searchAuthor, searchTitle)" />
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-4">
        <span class="p-float-label">
          <InputText id="test-button" type="text" v-model="geniusToken" v-tooltip="'visit docs.genius.com or genius.com/api-clients for creating an API key.'" />
          <label for="test-button">Genius Token</label>
          <Button label="Save Genius Token" class="p-button-success" @click="saveToken" />
          <Button label="Save Config to File" class="p-button-success" @click="saveConfig" />
          <Button label="Previous Verse" class="p-button-help" @click="previousVerse" v-tooltip.bottom="'You can use the left arrow key.'" />
          <Button label="Next Verse" class="p-button-help" @click="nextVerse" v-tooltip.bottom="'You can use the right arrow key.'" />
        </span>
      </div>
      <div class="col-12">
        <SongEditor v-model="songAddition" @add-song="addSong" />
      </div>
    </div>
    <Toast />
  </div>
</template>

<style scoped>
.max-vh {
  height: 100vh;
}
</style>
