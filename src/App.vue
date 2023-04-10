<script lang="ts">
import {onMounted, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api";
import SongList from "./components/SongList.vue";
import {useToast} from "primevue/usetoast";
import {register} from "@tauri-apps/api/globalShortcut";
import SongEditor from "./components/SongEditor.vue";
import SelectionDisplay from "./components/SelectionDisplay.vue";
import {useSettingsStore} from "./stores/settingsStore";
import {useSongListStore} from "./stores/songListStore";
import {useDisplaySelectionStore} from "./stores/displaySelectionStore";


export default {
  components: {SelectionDisplay, SongEditor, SongList},
  setup() {
    const toast = useToast();

    const settings = useSettingsStore();
    const songList = useSongListStore();
    const displaySelection = useDisplaySelectionStore();

    const selectedSong = ref([]);

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
          displaySelection.nextVerse();
        });
        await register('left', () => {
          displaySelection.previousVerse();
        });
      } catch (error) {
        console.error(error);
      }

      const result: any = await invoke("get_songs", {});
      songList.songs = result.songs;

      await settings.load();
      await displaySelection.load();
    })

    async function addSearchedSong(author: string, title: string) {
      try {
        toast.add({
          severity: "info",
          summary: "Searching",
          detail: `Searching for ${author} - ${title}.`,
          life: 3000,
        });
        // const result: any = await invoke('add_searched_song', { author, title });
        await songList.addSearchedSong(author, title);
        toast.add({
          severity: "success",
          summary: "Song Added",
          detail: `${author} - ${title} was added.`,
          life: 3000,
        });
      } catch (error) {
        console.error(error);
        toast.add({
          severity: "error",
          summary: "Failed to load song",
          detail: error,
          life: 3000,
        })
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
        await songList.addSong(songAddition.value);
        toast.add({
          severity: "success",
          summary: "Addition successful",
          detail: "Added song to song list.",
          life: 3000,
        });
      } catch (error) {
        console.error(error);
        toast.add({
          severity: "error",
          summary: "Addition failed",
          detail: "Failed to update the song list.",
          life: 3000,
        });
      }
    }

    watch(selectedSong, (currentValue, oldValue) => {
      console.log("old selected", oldValue);
      console.log("new selected", currentValue);
    })

    return {
      songList,
      selectedSong,
      settings,
      displaySelection,
      searchTitle,
      searchAuthor,
      songAddition,
      sidebarVisible,
      onMounted,
      addSearchedSong,
      addSong,
    }
  }
}
</script>

<template>
  <Sidebar v-model:visible="sidebarVisible" position="right" close-icon="">
    <h2>Settings</h2>
    <span class="flex flex-column settings-form-group">
      <label for="geniustoken-input">Genius API Token</label>
      <InputText id="geniustoken-input" type="text" placeholder="API Token" :model-value="settings.geniusToken" @blur="settings.setToken($event.target.value)" v-tooltip.left="'visit docs.genius.com or genius.com/api-clients for creating an API key.'" />
      <small>An API token is required for searching songs.</small>
      <small>Create an API token at <a href="https://genius.com/api-clients" target="_blank">genius.com</a>.</small>
    </span>
    <span class="flex flex-column settings-form-group">
      <label for="fontsize-input">Presentation Font Size</label>
      <InputText id="fontsize-input" type="text" placeholder="Font Size" :model-value="settings.fontSize" @blur="settings.setFontSize($event.target.value)" />
      <small>The font size used for the presentation text.</small>
      <small>A sensible default is '2.5rem'.</small>
      <small>You can also specify values in em or px.</small>
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
        <a href="javascript:void(0)">
          <i class="pi pi-cog topbar-icon" @click="sidebarVisible = !sidebarVisible"></i>
        </a>
      </div>
    </div>
  </div>
  <div class="main-body surface-ground px-4 md:px-6 lg:px-8">
    <div class="text-900 font-bold text-6xl mb-4 text-center">Beamer Software</div>
    <div class="grid">
      <div class="col-12 lg:col-8">
        <div class="p-3 h-full">
          <div class="shadow-2 p-3 h-full flex flex-column surface-card">
            <SongList :model-value="songList.songs" @update:model-value="songList.updateBackend($event)" @update:song-selection="selectedSong = $event" />
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
                <Button label="Remove First Song" class="p-button-danger" @click="songList.removeFirst" />
              </div>
              <div class="col-6">
                <Button label="Add Searched Song" class="p-button-success" @click="addSearchedSong(searchAuthor, searchTitle)" />
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="col-4">
      </div>
      <div class="col-12">
        <SongEditor v-model="songAddition" @add-song="addSong" />
      </div>
    </div>
    <div class="bottom-bar">
      <SelectionDisplay />
    </div>
    <Toast />
  </div>
</template>

<style scoped>
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

.settings-form-group {
  margin-bottom: 20px;
}

.main-body {
  padding-top: 5rem;
  padding-bottom: 160px;
}

.bottom-bar {
  position: fixed;
  background-color: #dbb57f;
  left: 0;
  bottom: 0;
  width: 100%;
  /*padding-left: 10rem;*/
  /*padding-right: 10rem;*/
  padding: 5px 5rem;
  /*border-top: 1px solid #999999;*/
  /*box-shadow: 0 0 2px 2px #dbb57f;*/
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.03), 0 0 2px rgba(0, 0, 0, 0.06), 0 0 6px rgba(0, 0, 0, 0.12);
  /*color: #ffffff;*/
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
