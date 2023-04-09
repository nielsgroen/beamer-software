import { defineStore } from "pinia";

export const useSettingsStore = defineStore('settings', {
    state: () => ({
        geniusToken: "",
    }),
    actions: {
        setGeniusToken(token: string) {
            this.geniusToken = token;
        },
    }
})
