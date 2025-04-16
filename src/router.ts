import About from '@/pages/About.vue'
import AppList from '@/pages/AppList.vue'
import Installation_Config from '@/pages/Installation/Config.vue'
import Installation from '@/pages/Installation/Index.vue'
import Installation_Progress from '@/pages/Installation/Progress.vue'
import Settings from '@/pages/Settings.vue'
import { useInstallationConfigStore } from '@/stores/installation_config'
import type { Router, RouteRecordRaw } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'

// Empty component for route redirection
const Dummy = { render: () => null }

// Route configuration with metadata
const routes = [
  { path: '/', redirect: '/Installation' },
  { path: '/Installation', component: Dummy },
  {
    path: '/Installation/Home',
    component: Installation,
    meta: {
      icon: 'mir-install_desktop',
    },
  },
  {
    path: '/Installation/Config',
    component: Installation_Config,
    meta: {
      icon: 'mir-settings_applications',
    },
  },
  {
    path: '/Installation/Progress',
    component: Installation_Progress,
    meta: {
      icon: 'mir-pending_actions',
    },
  },
  {
    path: '/AppList',
    component: AppList,
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
  {
    path: '/About',
    component: About,
    meta: {
      icon: 'mir-info',
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

    const installationConfig = useInstallationConfigStore()

    // Handle installation wizard navigation based on current page state
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
      }
      return { path }
    }

    // Reset installation state when returning to home page
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
