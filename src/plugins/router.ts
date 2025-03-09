// Core imports
import { useInstallationConfigStore } from '@/stores/installation_config'
import type { Router, RouteRecordRaw } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'

// Page components
import AppList from '@/AppList.vue'
import Installation from '@/Installation.vue'
import Installation_Config from '@/Installation/Config.vue'
import Installation_Progress from '@/Installation/Progress.vue'
import Settings from '@/Settings.vue'

const Dummy = { render: () => null }

// Route definitions
const routes = [
  { path: '/', redirect: '/Installation' },
  { path: '/Installation', component: Dummy },
  {
    path: '/Installation/Home',
    component: Installation,
    meta: {
      icon: 'mir install_desktop',
    },
  },
  {
    path: '/Installation/Config',
    component: Installation_Config,
    meta: {
      icon: 'mir settings_applications',
    },
  },
  {
    path: '/Installation/Progress',
    component: Installation_Progress,
    meta: {
      icon: 'mir pending_actions',
    },
  },
  {
    path: '/AppList',
    component: AppList,
    meta: {
      icon: 'mir apps',
    },
  },
  {
    path: '/Settings',
    component: Settings,
    meta: {
      icon: 'mir settings',
    },
  },
] as unknown as RouteRecordRaw[]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export function setupRouterGuards(router: Router) {
  router.beforeEach(async (to, from) => {
    // Skip guard for same route or direct navigation
    if (to.path === from.path) {
      return true
    }

    const installationConfig = useInstallationConfigStore()

    // Auto redirect for '/Installation' based on installationConfig.page
    if (to.path === '/Installation') {
      let path = ''
      switch (installationConfig.page) {
        case 'Home':
          path = '/Installation/Home'
          break
        case 'Config':
          path = '/Installation/Config'
          break
        case 'Progress':
          path = '/Installation/Progress'
          break
        case 'Finish':
          path = '/Installation/Progress'
          break
      }
      return { path }
    }

    // Clear data only when navigating to '/Installation/Home'
    if (to.path === '/Installation/Home') {
      installationConfig.$reset()
    }

    return true
  })
}

export function goTo(path: string) {
  router.push(path)
}

export default router
