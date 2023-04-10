<script lang="ts">

export default {
  props: ["modelValue", "fontSize"],
}

</script>

<template>
  <div class="verse-container" v-if="modelValue !== null">
    <p class="verse-line" v-for="line in modelValue.lines" :style="{ fontSize: fontSize }">
      {{ line }}
    </p>
  </div>
</template>

<style scoped>
.verse-container {
  padding: 30px 70px;
}
</style>
