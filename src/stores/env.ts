import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import type { Settings } from './settings'

interface Env {
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

    async setInitialSettings() {
      const { SettingsStore } = await import('./settings')
      const settings = SettingsStore()

      this.initialSettings = JSON.parse(JSON.stringify(settings.$state))
    },
  },
})
