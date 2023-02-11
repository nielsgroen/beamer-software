<script lang="ts">

import {computed, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api";
import OrderList from "primevue/orderlist";
// import { useModelWrapper } from "../js/modelwrapper.js";

// const props = defineProps(["songsList"])
//
// onMounted(async () => {
//     const songs = await invoke("get_songs", {});
//
//     console.log(songs);
//     songsList.value = songs;
// })


export default {
  components: { OrderList },
  props: ["modelValue", "songSelection"],
  // props: {
  //   modelValue: {
  //     type: Array,
  //     default: () => [],
  //   }
  // },
  setup(props, { emit }) {
    const songList = ref([]);

    // function bubbleReorder(reorderEvent) {
    //   emit('update:modelValue', reorderEvent.value);
    // }
    //
    // function bubbleSelected(selectionEvent) {
    //   console.log("selected", selectionEvent.value);
    // }
    // onMounted(async () => {
    //   songList.value = await invoke("get_songs", {});
    //   console.log("songList", songList);
    // })

    return {
      songList,
      // bubbleReorder,
    }
  }
}


// export default {
//   props: ["modelValue"],
//   setup(props, { emit }) {
//     return {
//       songsList: useModelWrapper(props, emit, 'modelValue')
//     }
//   },
//   async mounted() {
//     this.songsList = await invoke("get_songs", {})
//   }
// }
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