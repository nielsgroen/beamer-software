<script setup lang="ts">
// import HelloWorld from './components/HelloWorld.vue'
import {ref} from "vue";
import SongEditor from "./components/SongEditor.vue";
import {invoke} from "@tauri-apps/api";

const text = ref();
// const hello = null;

// invoke('get_lyrics', { author: "Justin Bieber", title: "Baby" })
//     .then((response) => {
//       hello = response;
//     });
</script>

<template>
  <div class="grid">
    <div class="col">
      <SongEditor />
    </div>
    <div class="col">
      <span class="p-float-label">
        <InputText id="test-button" type="text" v-model="text" />
        <label for="test-button">TestyMcTestface</label>
      </span>
    </div>

  </div>
<!--  <HelloWorld msg="Vite + Vue" />-->
</template>

<style scoped>

</style>
