import Home from '@/pages/Home.vue'
import AppConfig from '@/pages/Install/App/AppConfig.vue'
import AppProgress from '@/pages/Install/App/AppProgress.vue'
import ToolConfig from '@/pages/Install/Tool/ToolConfig.vue'
import ToolProgress from '@/pages/Install/Tool/ToolProgress.vue'
import Library from '@/pages/Library/Index.vue'
import Settings from '@/pages/Settings.vue'
import type { Router, RouteRecordRaw } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'
import { generalStore, installConfig } from './main'

const Dummy = { render: () => null }

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
] as RouteRecordRaw[]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export function setupRouterGuards(router: Router) {
  router.beforeEach(async (to, from) => {
    if (to.path === from.path) {
      return true
    }

    if (to.path === '/Install') {
      let path = ''
      switch (generalStore.page) {
        case 'Home':
          path = '/Home'
          break
        case 'Install_App_Config':
          path = '/Install/App/Config'
          break
        case 'Install_Tool_Config':
          path = '/Install/Tool/Config'
          break
        case 'Install_App_Progress':
          path = '/Install/App/Progress'
          break
        case 'Install_Tool_Progress':
          path = '/Install/Tool/Progress'
          break
      }
      return { path }
    }

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
