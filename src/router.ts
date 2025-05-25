import AppList from '@/pages/AppList.vue'
import CopyOnly from '@/pages/CopyOnly/Config.vue'
import CopyOnlyProgress from '@/pages/CopyOnly/Progress.vue'
import Home from '@/pages/Home.vue'
import Installation_Config from '@/pages/Installation/Config.vue'
import Installation_Progress from '@/pages/Installation/Progress.vue'
import Settings from '@/pages/Settings.vue'
import type { Router, RouteRecordRaw } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'
import { useInstallConfigStore } from './stores/install_config'

const installConfig = useInstallConfigStore()

// Empty component for route redirection
const Dummy = { render: () => null }

// Route configuration with metadata
const routes = [
  { path: '/', redirect: '/Installation' },
  { path: '/Installation', component: Dummy },
  {
    path: '/Home',
    component: Home,
    meta: {
      icon: 'mir-install_desktop',
    },
  },
  {
    path: '/Installation/App/Config',
    component: Installation_Config,
    meta: {
      icon: 'mir-settings_applications',
    },
  },
  {
    path: '/Installation/App/Progress',
    component: Installation_Progress,
    meta: {
      icon: 'mir-pending_actions',
    },
  },
  {
    path: '/Installation/Lib/Config',
    component: CopyOnly,
    meta: {
      icon: 'mir-folder_copy',
    },
  },
  {
    path: '/Installation/Lib/Progress',
    component: CopyOnlyProgress,
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
    if (to.path === '/Installation') {
      let path = ''
      switch (installConfig.page) {
        case 'Home':
          path = '/Home'
          break
        case 'Install_App_Config':
          path = '/Installation/App/Config'
          break
        case 'Install_App_Progress':
          path = '/Installation/App/Progress'
          break
        case 'Install_Lib_Config':
          path = '/Installation/Lib/Config'
          break
        case 'Install_Lib_Progress':
          path = '/Installation/Lib/Progress'
          break
      }
      return { path }
    }

    // Reset installation state when returning to home page
    if (to.path === '/Home') {
      installConfig.$reset()
    }

    return true
  })
}

export function goTo(path: string) {
  router.push(path)
}

export default router
