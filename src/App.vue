<script lang="ts">
import {onMounted, ref, watch} from "vue";
import SongEditor from "./components/SongEditor.vue";
import {invoke} from "@tauri-apps/api";
import SongList from "./components/SongList.vue";
import {useToast} from "primevue/usetoast";
import Toast from 'primevue/toast';

export default {
  components: {SongList},
  setup() {
    const toast = useToast();

    const songList = ref([]);
    const selectedSong = ref([]);
    const geniusToken = ref("");

    onMounted(async () => {
      const result: Array = await invoke("get_songs", {});
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
        const result: Array = await invoke('add_searched_song', { author, title });
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

    async function processClientSongListUpdate(newSongList) {
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
      processClientSongListUpdate,
      onMounted,
      removeFirst,
      addSearchedSong,
      saveToken,
      saveConfig,
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
                <Button label="Remove First Song" class="p-button-danger" @click="removeFirst" />
              </div>
              <div class="col-6">
                <Button label="Add Searched Song" class="p-button-success" @click="addSearchedSong('justin bieber', 'baby')" />
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-4">
        <span class="p-float-label">
          <InputText id="test-button" type="text" v-model="geniusToken" />
          <label for="test-button">Genius Token</label>
          <Button label="Save Genius Token" class="p-button-success" @click="saveToken" />
          <Button label="Save Config to File" class="p-button-success" @click="saveConfig" />
        </span>
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
