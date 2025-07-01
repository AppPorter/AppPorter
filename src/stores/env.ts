import { exec } from '@/exec'
import { defineStore } from 'pinia'
import { Env } from '#/Env'

export const EnvStore = defineStore('env', {
  state: (): Env => ({
    debug: false,
    elevated: false,
    system_drive_letter: '',
    user_sid: '',
    username: '',
  }),

  actions: {
    async loadEnv() {
      this.$patch(await exec<Env>('LoadEnv'))
    },
  },
})
