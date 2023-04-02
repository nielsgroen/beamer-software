<script lang="ts">
import {invoke} from "@tauri-apps/api";
import {ref} from "vue";

export default {
  props: ["modelValue"],
}

</script>

<template>
  <div class="p-3 h-full">
    <div class="shadow-2 p-3 h-full flex flex-column surface-card">
      <h3>Song Editor</h3>
      <div class="formgrid grid">
        <div class="field col-6">
          <span class="p-float-label w-full p-1">
            <InputText id="edit-title" v-model="modelValue.title" type="text" class="w-full" />
            <label for="edit-title">Title</label>
          </span>
        </div>
        <div class="field col-6">
          <span class="p-float-label w-full p-1">
            <InputText id="edit-author" v-model="modelValue.author" type="text" class="w-full" />
            <label for="edit-author">Author</label>
          </span>
        </div>
        <div class="field col-12">
          <span class="p-float-label w-full p-1">
            <Textarea id="edit-text" v-model="modelValue.songText" rows="10" class="w-full" style="resize: none;"/>
            <label for="edit-text">Song Text</label>
          </span>
        </div>
      </div>
      <Button label="Add Song" class="p-button-success m-3" @click="$emit('addSong', $event)" />
      <Panel header="Tooltip">
        <p class="m-0">
          Verses will be split on empty lines.
          To contain an empty line within a verse insert a &ltspace&gt on that line.
        </p>
      </Panel>
    </div>
  </div>
</template>
