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
    const testText = ref("henk");

    onMounted(async () => {
      const result: Array = await invoke("get_songs", {});
      songList.value = result.songs;
      console.log("songList mounted", songList);
    })

    async function removeFirst() {
      const newSongList = songList.value;
      newSongList.shift();
      await processClientSongListUpdate(newSongList);
    }

    async function addSearchedSong(author: String, title: String) {
      try {
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
        console.log("error called");
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
      testText,
      processClientSongListUpdate,
      onMounted,
      removeFirst,
      addSearchedSong,
    }
  }
}
</script>

<template>
  <div class="grid">
    <div class="col-8">
<!--      <SongEditor v-model="songList" />-->
      <SongList :model-value="songList" @update:model-value="processClientSongListUpdate($event)" @update:song-selection="selectedSong = $event" />
    </div>
    <div class="col-4">
      <span class="p-float-label">
        <InputText id="test-button" type="text" v-model="testText" />
        <label for="test-button">TestyMcTestface</label>
      </span>
      <button @click="removeFirst">Clicky</button>
      <button @click="addSearchedSong('justin bieber', 'baby')">Clicky</button>
    </div>
  </div>
  <Toast />
</template>

<style scoped>

</style>
