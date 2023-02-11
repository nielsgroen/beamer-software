<script lang="ts">
// import HelloWorld from './components/HelloWorld.vue'
import {onMounted, ref, watch} from "vue";
import SongEditor from "./components/SongEditor.vue";
import {invoke} from "@tauri-apps/api";
import SongList from "./components/SongList.vue";

// const text = ref("hi");
// const hello = null;

// invoke('get_lyrics', { author: "Justin Bieber", title: "Baby" })
//     .then((response) => {
//       hello = response;
//     });

export default {
  components: {SongList},
  setup() {
    const songList = ref([]);
    const selectedSong = ref([]);
    const testText = ref("henk");


    onMounted(async () => {
      songList.value = await invoke("get_songs", {});
      console.log("songList mounted", songList);
    })

    function resetSelection() {
      selectedSong.value = [];
    }

    function removeFirst() {
      songList.value.shift();
    }

    watch(songList, (currentValue, oldValue) => {
      console.log("old val", oldValue);
      console.log("new val", currentValue);
    });

    watch(selectedSong, (currentValue ,oldValue) => {
      console.log("old selected", oldValue);
      console.log("new selected", currentValue);
    })

    return {
      songList,
      selectedSong,
      testText,
      onMounted,
      removeFirst,
    }
  }
}
</script>

<template>
  <div class="grid">
    <div class="col-8">
<!--      <SongEditor v-model="songList" />-->
      <SongList v-model="songList" @update:song-selection="selectedSong = $event" />
    </div>
    <div class="col-4">
      <span class="p-float-label">
        <InputText id="test-button" type="text" v-model="testText" />
        <label for="test-button">TestyMcTestface</label>
      </span>
      <button @click="removeFirst">Clicky</button>
    </div>
<!--    <div class="col-8">-->
<!--      <SongList />-->
<!--    </div>-->
  </div>
</template>

<style scoped>

</style>
