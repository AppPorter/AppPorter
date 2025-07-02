import { defineStore } from 'pinia'
import { Settings } from '#/Settings'

type Pages =
  | 'Home'
  | 'Install_App_Config'
  | 'Install_App_Progress'
  | 'Install_Tool_Config'
  | 'Install_Tool_Progress'

interface Drawer {
  preview: boolean
  uninstall: boolean
}

interface General {
  page: Pages
  drawer: Drawer

  initialSettings: Settings | null
  isBasicSettingsChanged: boolean
  isDarkMode: boolean
}

export const GeneralStore = defineStore('general', {
  state: (): General => ({
    page: 'Home',
    drawer: {
      preview: false,
      uninstall: false,
    },

    initialSettings: null,
    isBasicSettingsChanged: false,
    isDarkMode: window.matchMedia('(prefers-color-scheme: dark)').matches,
  }),

  actions: {
    async setInitialSettings() {
      const { SettingsStore } = await import('./settings')
      const settings = SettingsStore()

      this.initialSettings = settings.$state
    },
  },
})
