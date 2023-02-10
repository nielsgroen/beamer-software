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
  <div class="container">
    <div class="row">
      <div class="two-thirds column">
        <SongEditor />
      </div>
      <div class="one-third column">
        <span class="p-float-label">
          <InputText id="test-button" type="text" v-model="text" />
          <label for="test-button">TestyMcTestface</label>
        </span>
      </div>
    </div>

  </div>
<!--  <HelloWorld msg="Vite + Vue" />-->
</template>

<style scoped>

</style>
