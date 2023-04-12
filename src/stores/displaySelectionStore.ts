import { defineStore } from "pinia";
import {invoke} from "@tauri-apps/api";

export const useDisplaySelectionStore = defineStore('displaySelection', {
    state: () => ({
        currentDisplay: {
            slot_id: 0,
            slot_position: 0,
            song: {
                author: "None",
                title: "Empty Panel",
                verses: {
                    lines: [],
                },
            },
            verse_num: 0,
        },
        nextDisplay: {
            slot_id: 0,
            slot_position: 0,
            song: {
                author: "None",
                title: "Empty Panel",
                verses: {
                    lines: [],
                },
            },
            verse_num: 0,
        },
    }),
    actions: {
        async load() {
            const result: any = await invoke("get_display_selection", {});

            console.log("load result", result);
            this.currentDisplay = result[0];
            this.nextDisplay = result[1];
        },
        async nextVerse() {
            await invoke("next_verse", {});
        },
        async previousVerse() {
            await invoke("previous_verse", {});
        },
    }
})
