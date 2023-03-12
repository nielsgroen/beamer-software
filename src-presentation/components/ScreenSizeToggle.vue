<script lang="ts">


</script>

<template>
  <div
    id="fullscreen-toggle"
    @click="$emit('changeFullScreen', $event)"
  >
    <i class="pi pi-fw pi- pi-clone"></i>
  </div>
</template>

<style scoped>

#fullscreen-toggle {
  visibility: hidden;
  opacity: 0;
  position: fixed;
  left: 40px;
  bottom: 40px;
  border: 2px solid #777777;
  background-color: #f9f9f9;
  border-radius: 15px;
  padding: 12px;
  color: #777777;
  transition: all 0.3s;
  cursor: pointer;
}

#fullscreen-toggle:hover {
  background-color: #efefef;
  border-color: #606060;
  color: #606060;
}

#fullscreen-toggle > i {
  font-size: 2rem;
}

#presentation-app:hover #fullscreen-toggle {
  visibility: visible;
  opacity: 1;
}

</style>
