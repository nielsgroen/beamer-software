<script lang="ts">
import {useDisplaySelectionStore} from "../stores/displaySelectionStore";

export default {
  setup() {
    const displaySelection = useDisplaySelectionStore();

    return {
      displaySelection,
    }

    // TODO: make sure this updates when songlist changes
  }
}
</script>

<template>
  <div class="grid" v-if="displaySelection.currentDisplay != null">
    <div class="hidden lg:block lg:col-2">
      <a href="javascript:void(0)" @click="displaySelection.previousVerse">
        <i class="display-navigation-button left-display-navigation-button pi pi-backward" v-tooltip.top="'You can use the arrow keys.'"></i>
      </a>
    </div>
    <div class="lg:col-4 col-6">
      <p class="display-header">
        Current
      </p>
      <p class="song-name">
        {{ displaySelection.currentDisplay.song.author }} - {{ displaySelection.currentDisplay.song.title }}
      </p>
      <small>
        Song {{ displaySelection.currentDisplay.slot_position }}, Verse {{ displaySelection.currentDisplay.verse_num }}
      </small>
      <p class="verse-line" v-if="displaySelection.currentDisplay.song.verses.length > 0">
        {{ displaySelection.currentDisplay.song.verses[displaySelection.currentDisplay.verse_num].lines[0] }}
      </p>
    </div>
    <div class="lg:col-4 col-6">
      <p class="display-header">
        Next
      </p>
      <p class="song-name">
        {{ displaySelection.nextDisplay.song.author }} - {{ displaySelection.nextDisplay.song.title }}
      </p>
      <small>
        Song {{ displaySelection.nextDisplay.slot_position }}, Verse {{ displaySelection.nextDisplay.verse_num }}
      </small>
      <p class="verse-line" v-if="displaySelection.nextDisplay.song.verses.length > 0">
        {{ displaySelection.nextDisplay.song.verses[displaySelection.nextDisplay.verse_num].lines[0] }}
      </p>
    </div>
    <div class="hidden lg:block lg:col-2">
      <a href="javascript:void(0)" @click="displaySelection.nextVerse">
        <i class="display-navigation-button right-display-navigation-button pi pi-forward" v-tooltip.top="'You can use the arrow keys.'"></i>
      </a>
    </div>
  </div>
</template>

<style scoped>
.display-header {
  font-size: 0.8rem;
  font-weight: bold;
}

.song-name {
  margin: 0.5rem 0;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.verse-line {
  margin: 0.5rem 0;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.display-navigation-button {
  font-size: 1.5rem;
  margin: 2rem;
  padding: 0.7rem;
  border-radius: 5rem;
  /*border: 1px solid #000;*/
  background-color: #c29f6e;
  color: #000000;
}

.display-navigation-button.left-display-navigation-button {
  margin-left: auto;
  margin-right: 1.5rem;
  float: right;
  padding: 0.7rem 0.8rem 0.7rem 0.6rem;
}

.display-navigation-button.right-display-navigation-button {
  margin-left: 1.5rem;
  margin-right: auto;
  padding: 0.7rem 0.6rem 0.7rem 0.8rem;
}
</style>
