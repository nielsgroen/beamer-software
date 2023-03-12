<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {ref} from "vue";

// type Song = {
//   title: String,
//   author: String,
//   verses: String[],
// }

const song = ref('');
// song.value = await invoke('greet', { name: 'Pjiet' });

async function handleClick() {
  song.value = await invoke('greet', { name: 'Pjiet' });
}

async function get_lyrics() {
  song.value = await invoke('get_lyrics', { author: "Justin Bieber", title: "Baby" });
}


// invoke('greet', { name: 'Fronk' })
//     .then((response) => {
//         hello.value = response;
//     });

</script>

<template>
  <h2>Song Editor</h2>
  <button @click="get_lyrics">Pjot</button>
  <p>{{ song }}</p>

  <div v-if="song">
    <div v-for="verse in song.verses">
      <p v-for="line in verse.lines">{{ line }}</p>
    </div>
  </div>

</template>