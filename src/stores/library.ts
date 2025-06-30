import { exec } from '@/exec'
import { defineStore } from 'pinia'
import type { Library } from '#/Library'

export type InstallTypes = 'app' | 'tool' | 'url'

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

    hasLink(url: string): boolean {
      return this.apps.some((app) => app.url === url) || this.tools.some((tool) => tool.url === url)
    },

    hasApp(url: string): boolean {
      return this.apps.some((app) => app.url === url)
    },

    hasTool(url: string): boolean {
      return this.tools.some((tool) => tool.url === url)
    },

    getAppByTimestamp(timestamp: bigint) {
      return this.installedApps.find((app) => app.timestamp === timestamp)
    },

    getToolByTimestamp(timestamp: bigint) {
      return this.installedTools.find((tool) => tool.timestamp === timestamp)
    },

    async executeUninstall(apptype: InstallTypes, timestamp: number) {
      switch (apptype) {
        case 'app':
          await exec('UninstallApp', { timestamp })
          break
        case 'tool':
          await exec('UninstallTool', { timestamp })
          break
      }

      await this.loadLibrary()
    },

    async removeApp(timestamp: bigint) {
      this.apps = this.apps.filter((app) => app.timestamp !== timestamp)
      await this.saveLibrary()
    },

    async removeTool(timestamp: bigint) {
      this.tools = this.tools.filter((tool) => tool.timestamp !== timestamp)
      await this.saveLibrary()
    },
  },
})
