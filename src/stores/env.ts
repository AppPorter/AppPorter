import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

interface Env {
  first_run: boolean
  debug: boolean
  elevated: boolean
  system_drive_letter: string
  user_sid: string
  username: string
}

export const useEnvStore = defineStore('env', {
  state: (): Env => ({
    first_run: true,
    debug: false,
    elevated: false,
    system_drive_letter: '',
    user_sid: '',
    username: '',
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

    async acknowledgeFirstRun() {
      this.first_run = false
      await this.saveEnv()
    },
  },
})
