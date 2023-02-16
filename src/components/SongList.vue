<script lang="ts">

import {computed, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api";
import OrderList from "primevue/orderlist";

export default {
  components: { OrderList },
  props: ["modelValue", "songSelection"],
  setup(props, { emit }) {
    const songList = ref([]);

    return {
      songList,
    }
  }
}

</script>

<template>
  <div class="card">
    <OrderList
        :model-value="modelValue"
        @update:model-value="$emit('update:modelValue', $event)"
        @update:selection="$emit('update:songSelection', $event)"
        list-style="height:auto"
        data-key="id"
    >
      <template #header>
        Song list
      </template>
      <template #item="songSlot">
        <div v-if="'Empty' !== songSlot.item.slot" class="p-caritem">
          <h6>{{ songSlot.item.slot.Song.title }}</h6>
          <i class="pi">{{ songSlot.item.slot.Song.author }}</i>
        </div>
        <div v-else class="p-caritem">
          <h6>Empty Slide</h6>
        </div>
      </template>
    </OrderList>
  </div>
</template>

<style scoped>

</style>