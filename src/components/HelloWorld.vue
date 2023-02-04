<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps<{ msg: string }>()

const count = ref(0)
let hello = "World";

// invoke('greet', { name: 'Fronk' })
//     .then((response) => {
//         hello = response;
//     });

invoke('get_lyrics', { author: "Justin Bieber", title: "Baby" })
    .then((response) => {
        hello = response;
    });

</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}, hello!</button>
    <p v-for="item in hello">
      {{ item }}
    </p>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
