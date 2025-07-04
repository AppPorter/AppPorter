import type { App } from '#/App'
import type { Library } from '#/Library'
import type { Tool } from '#/Tool'
import { exec } from '@/exec'
import { defineStore } from 'pinia'

export type ItemTypes = 'app' | 'tool' | 'url'

export const LibraryStore = defineStore('library', {
  state: (): Library => ({
    apps: [],
    tools: [],
    urls: [],
  }),

  getters: {
    installedApps: (state) => {
      return state.apps.filter((app) => app.installed)
    },
    installedTools: (state) => {
      return state.tools.filter((tool) => tool.installed)
    },
  },

  actions: {
    async loadLibrary() {
      const result = await exec<Library>('LoadLibrary')
      this.$patch(result)
    },
    async saveLibrary() {
      await exec('SaveLibrary', {
        library: this.$state,
      })
    },

    async hasLink(url: string): Promise<boolean> {
      return await exec<boolean>('HasLink', { url })
    },

    async getApp(id: string) {
      return await exec<App>('GetApp', { id })
    },
    async getTool(id: string) {
      return await exec<Tool>('GetTool', { id })
    },
    async getById(id: string) {
      const app = this.apps.find((a) => a.id === id)
      if (app) return app
      const tool = this.tools.find((t) => t.id === id)
      if (tool) return tool
      return null
    },

    async executeUninstall(apptype: ItemTypes, id: string) {
      switch (apptype) {
        case 'app':
          await exec('UninstallApp', { id })
          break
        case 'tool':
          await exec('UninstallTool', { id })
          break
      }

      await this.loadLibrary()
    },

    async remove(id: string) {
      await exec('Remove', { id })
      await this.loadLibrary()
    },
  },
})
