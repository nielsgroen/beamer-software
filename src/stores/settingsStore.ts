import { defineStore } from "pinia";
import {invoke} from "@tauri-apps/api";

export const useSettingsStore = defineStore('settings', {
    state: () => ({
        geniusToken: "",
    }),
    actions: {
        async setToken() {
            await invoke("set_genius_token", { newToken: this.geniusToken });
        },

        async saveSettings() {
            await invoke("save_config", {});
        },
    }
})
