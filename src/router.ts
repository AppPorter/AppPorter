import AppList from '@/pages/AppList.vue'
import Install_Lib_Config from '@/pages/CopyOnly/Config.vue'
import Install_Lib_Progress from '@/pages/CopyOnly/Progress.vue'
import Home from '@/pages/Home.vue'
import Install_Config from '@/pages/Installation/Config.vue'
import Install_Progress from '@/pages/Installation/Progress.vue'
import Settings from '@/pages/Settings.vue'
import type { Router, RouteRecordRaw } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'
import { InstallConfigStore } from './stores/install_config'

const installConfig = InstallConfigStore()

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
    component: Install_Config,
    meta: {
      icon: 'mir-settings_applications',
    },
  },
  {
    path: '/Install/App/Progress',
    component: Install_Progress,
    meta: {
      icon: 'mir-pending_actions',
    },
  },
  {
    path: '/Install/Lib/Config',
    component: Install_Lib_Config,
    meta: {
      icon: 'mir-folder_copy',
    },
  },
  {
    path: '/Install/Lib/Progress',
    component: Install_Lib_Progress,
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
    if (to.path === '/Install') {
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
        case 'Install_Lib_Config':
          path = '/Install/Lib/Config'
          break
        case 'Install_Lib_Progress':
          path = '/Install/Lib/Progress'
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
