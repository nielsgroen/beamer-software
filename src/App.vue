<script lang="ts">
import {onMounted, ref, watch} from "vue";
// import SongEditor from "./components/SongEditor.vue";
import {invoke} from "@tauri-apps/api";
import SongList from "./components/SongList.vue";
import {useToast} from "primevue/usetoast";
import Toast from 'primevue/toast';
import {register} from "@tauri-apps/api/globalShortcut";
import SongEditor from "./components/SongEditor.vue";
import {useSettingsStore} from "./stores/settingsStore";


export default {
  components: {SongEditor, SongList},
  setup() {
    const toast = useToast();

    const songList = ref([]);
    const selectedSong = ref([]);
    const settings = useSettingsStore();

    const searchTitle = ref("");
    const searchAuthor = ref("");

    const songAddition = ref({
      title: "",
      author: "",
      songText: "",
    });

    const sidebarVisible = ref(false);

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
      settings.setGeniusToken(newToken);
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
      await invoke("set_genius_token", { newToken: settings.geniusToken });
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
      settings,
      searchTitle,
      searchAuthor,
      songAddition,
      sidebarVisible,
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
  <Sidebar v-model:visible="sidebarVisible" position="right" close-icon="">
    <h2>Settings</h2>
    <span class="p-float-label">
      <InputText id="test-button" type="text" v-model="settings.geniusToken" v-tooltip.left="'visit docs.genius.com or genius.com/api-clients for creating an API key.'" />
      <label for="test-button">Genius Token</label>
      <Button label="Save Genius Token" class="p-button-success" @click="saveToken" />
      <Button label="Save Settings to File" class="p-button-success" @click="saveConfig" />
    </span>
  </Sidebar>
  <div class="topbar">
    <div class="topbar-content">
      <div class="topbar-item">
        <a href="https://github.com/nielsgroen/beamer-software/releases" target="_blank" v-tooltip.bottom="'For finding the newest release, and reporting bugs.'">
          <i class="pi pi-github topbar-icon"></i>
        </a>
      </div>
      <div class="topbar-item">
        <a href="#">
          <i class="pi pi-cog topbar-icon" @click="sidebarVisible = !sidebarVisible"></i>
        </a>
      </div>
    </div>
  </div>
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

.topbar {
  position: fixed;
  top: 0;
  right: 0;
  height: 4rem;
  padding: 1rem 0;
  background-color: transparent;
  z-index: 2000;
}

.topbar-content {
  display: flex;
  justify-content: flex-end;
}

.topbar-content > * {
  margin-left: 1rem;
  margin-right: 1rem;
}

.topbar-icon {
  font-size: 2rem;
  color: #000000;
}
</style>

<style>
/*
Unscoped style to override sidebar close button showing up
 */
.p-sidebar-header .p-sidebar-close {
  display: none;
}
</style>
