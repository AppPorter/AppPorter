import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import type { Settings } from './settings'

interface Env {
  first_run: boolean
  debug: boolean
  elevated: boolean
  system_drive_letter: string
  user_sid: string
  username: string
  initialSettings: Settings | null
  isBasicSettingsChanged: boolean
  isDarkMode: boolean
}

export const EnvStore = defineStore('env', {
  state: (): Env => ({
    first_run: true,
    debug: false,
    elevated: false,
    system_drive_letter: '',
    user_sid: '',
    username: '',
    initialSettings: null,
    isBasicSettingsChanged: false,
    isDarkMode: window.matchMedia('(prefers-color-scheme: dark)').matches,
  }),

  actions: {
    async loadEnv() {
      const result = await invoke<string>('execute_command', {
        command: { name: 'LoadEnv' },
      })
      const env = JSON.parse(result)

      this.$patch(env)
    },

    async saveEnv() {
      await invoke('execute_command', {
        command: {
          name: 'SaveEnv',
          env: this.$state,
        },
      })
    },

    setInitialSettings(settings: Settings) {
      this.initialSettings = settings
    },

    async acknowledgeFirstRun() {
      this.first_run = false
      await this.saveEnv()
    },
  },
})
