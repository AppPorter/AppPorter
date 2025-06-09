import Home from '@/pages/Home.vue'
import AppProgress from '@/pages/Install/App/AppProgress.vue'
import Config from '@/pages/Install/Config.vue'
import LibProgress from '@/pages/Install/Lib/LibProgress.vue'
import Preview from '@/pages/Install/Preview.vue'
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
    path: '/Install/Preview',
    component: Preview,
    meta: {
      icon: 'mir-install_desktop',
    },
  },
  {
    path: '/Install/Config',
    component: Config,
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
    path: '/Install/Lib/Progress',
    component: LibProgress,
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
        case 'Preview':
          path = '/Install/Preview'
          break
        case 'Install_App_Config':
          path = '/Install/Config'
          break
        case 'Install_App_Progress':
          path = '/Install/App/Progress'
          break
        case 'Install_Lib_Config':
          path = '/Install/Config'
          break
        case 'Install_Lib_Progress':
          path = '/Install/Lib/Progress'
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
