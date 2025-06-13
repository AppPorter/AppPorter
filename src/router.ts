import Home from '@/pages/Home.vue'
import AppConfig from '@/pages/Install/App/AppConfig.vue'
import AppProgress from '@/pages/Install/App/AppProgress.vue'
import ToolConfig from '@/pages/Install/Tool/ToolConfig.vue'
import ToolProgress from '@/pages/Install/Tool/ToolProgress.vue'
import Library from '@/pages/Library/Index.vue'
import Settings from '@/pages/Settings.vue'
import type { Router, RouteRecordRaw } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'

// Empty component for route redirection
const Dummy = { render: () => null }

// Route configuration with metadata
const routes = [
  { path: '/', redirect: '/Install' },
  { path: '/Install', component: Dummy },
  {
    path: '/Home',
    component: Home,
    meta: {
      icon: 'mir-install_desktop',
    },
  },
  {
    path: '/Install/App/Config',
    component: AppConfig,
    meta: {
      icon: 'mir-settings_applications',
    },
  },
  {
    path: '/Install/Tool/Config',
    component: ToolConfig,
    meta: {
      icon: 'mir-settings_applications',
    },
  },
  {
    path: '/Install/App/Progress',
    component: AppProgress,
    meta: {
      icon: 'mir-pending_actions',
    },
  },
  {
    path: '/Install/Tool/Progress',
    component: ToolProgress,
    meta: {
      icon: 'mir-pending_actions',
    },
  },
  {
    path: '/Library',
    component: Library,
    meta: {
      icon: 'mir-apps',
    },
  },
  {
    path: '/Settings',
    component: Settings,
    meta: {
      icon: 'mir-settings',
    },
  },
] as unknown as RouteRecordRaw[]

// Create router instance with in-memory history mode
const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export function setupRouterGuards(router: Router) {
  router.beforeEach(async (to, from) => {
    if (to.path === from.path) {
      return true
    }

    // Handle installation wizard navigation based on current page state
    if (to.path === '/Install') {
      // Import store here to avoid accessing it before Pinia is initialized
      const { InstallConfigStore } = await import('./stores/install_config')
      const installConfig = InstallConfigStore()

      let path = ''
      switch (installConfig.page) {
        case 'Home':
          path = '/Home'
          break
        case 'Install_App_Config':
          path = '/Install/App/Config'
          break
        case 'Install_App_Progress':
          path = '/Install/App/Progress'
          break
        case 'Install_Tool_Config':
          path = '/Install/Tool/Config'
          break
        case 'Install_Tool_Progress':
          path = '/Install/Tool/Progress'
          break
      }
      return { path }
    }

    // Reset installation state when returning to home page
    if (to.path === '/Home') {
      // Import store here to avoid accessing it before Pinia is initialized
      const { InstallConfigStore } = await import('./stores/install_config')
      const installConfig = InstallConfigStore()
      installConfig.$reset()
    }

    return true
  })
}

export function goTo(path: string) {
  router.push(path)
}

export default router
