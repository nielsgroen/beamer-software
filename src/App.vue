<script setup lang="ts">
// import HelloWorld from './components/HelloWorld.vue'
import {ref} from "vue";

const text = ref();
</script>

<template>
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <span class="p-float-label">
    <InputText id="test-button" type="text" v-model="text" />
    <label for="test-button">TestyMcTestface</label>
  </span>
<!--  <HelloWorld msg="Vite + Vue" />-->
</template>

<style scoped>

</style>
