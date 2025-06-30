import { exec } from '@/exec'
import { defineStore } from 'pinia'
import { Env } from '#/Env'
import { Settings } from '#/Settings'

interface General {
  initialSettings: Settings | null
  isBasicSettingsChanged: boolean
  isDarkMode: boolean
}

export const EnvStore = defineStore('env', {
  state: (): Env & General => ({
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
      this.$patch(await exec<Env>('LoadEnv'))
    },

    async setInitialSettings() {
      const { SettingsStore } = await import('./settings')
      const settings = SettingsStore()

      this.initialSettings = JSON.parse(JSON.stringify(settings.$state))
    },
  },
})
