import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";

export const useSettingsStore = defineStore('settings', {
    state: () => ({
        geniusToken: "",
        fontSize: "",
    }),
    actions: {
        async load() {
            this.geniusToken = await invoke("get_genius_token", {});
            this.fontSize = await invoke("get_font_size", {});
        },

        async setToken(newToken: string) {
            this.geniusToken = newToken;
            await invoke("set_genius_token", { newToken: this.geniusToken });
            await invoke("save_config", {});
        },

        async setFontSize(newFontSize: string) {
            this.fontSize = newFontSize;
            await invoke("set_font_size", { newFontSize: this.fontSize });
            await invoke("save_config", {});
        },

        async saveSettings() {
            await invoke("save_config", {});
        },
    }
})
