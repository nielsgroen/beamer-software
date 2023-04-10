<script lang="ts">
import VerseDisplay from "./components/VerseDisplay.vue";
import ScreenSizeToggle from "./components/ScreenSizeToggle.vue";
import {ref} from "vue";
import {listen} from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

export default {
  components: {VerseDisplay, ScreenSizeToggle},
  setup() {
    const currentVerse = ref(null);
    const isFullScreen = ref(false);
    const fontSize = ref("2.5rem");

    listen('update-verse', (event) => {
      currentVerse.value = event.payload;
      console.log(event.payload);
    })

    listen('update-font-size', (event) => {
      fontSize.value = event.payload;
      console.log(event.payload);
    })

    console.log("setup run");

    async function toggleFullScreen() {
      isFullScreen.value = !isFullScreen.value;
      await appWindow.setFullscreen(isFullScreen.value);
    }

    return {
      currentVerse,
      fontSize,
      toggleFullScreen,
    }
  }
}
</script>

<template>
  <div id="presentation-app">
    <VerseDisplay v-model="currentVerse" :font-size="fontSize" />
    <ScreenSizeToggle @changeFullScreen="toggleFullScreen" />
  </div>
</template>

<style scoped>
#presentation-app {
  height: 100vh;
  width: 100%;
}
</style>

