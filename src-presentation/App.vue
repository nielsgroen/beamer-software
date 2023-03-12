<script lang="ts">
import VerseDisplay from "./components/VerseDisplay.vue";
import {ref} from "vue";
import {listen} from "@tauri-apps/api/event";

export default {
  components: {VerseDisplay},
  setup() {
    const currentVerse = ref(null);

    listen('update-verse', (event) => {
      currentVerse.value = event.payload;
      console.log(event.payload);
    })

    console.log("setup run")

    return {
      currentVerse,
    }
  }
}
</script>

<template>
  <div>
    <VerseDisplay v-model="currentVerse" />
  </div>
</template>

