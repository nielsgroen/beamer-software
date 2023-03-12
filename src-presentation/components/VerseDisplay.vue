<script lang="ts">

export default {
  props: ["modelValue"],
}

</script>

<template>
  <div class="verse-container" v-if="modelValue !== null">
    <p class="verse-line" v-for="line in modelValue.lines">
      {{ line }}
    </p>
  </div>
</template>

<style scoped>
.verse-container {
  padding: 30px 70px;
}

.verse-line {
  font-size: 2.5rem;
}
</style>
