// Core imports
import { useInstallationConfigStore } from '@/stores/installation_config'
import { useConfirm } from 'primevue/useconfirm'
import type { Router } from 'vue-router'
import { createMemoryHistory, createRouter } from 'vue-router'

// Page components
import AppList from '@/AppList.vue'
import Installation from '@/Installation.vue'
import Installation_Config from '@/Installation/Config.vue'
import Installation_Progress from '@/Installation/Progress.vue'
import Settings from '@/Settings.vue'

// Route definitions
const routes = [
  { path: '/', redirect: '/Installation' },
  {
    path: '/Installation',
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
] as const

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export function setupRouterGuards(router: Router) {
  router.beforeEach(async (to, from) => {
    // Skip guard if both routes are the same
    if (to.path === from.path) {
      return true
    }

    const installationConfig = useInstallationConfigStore()

    // Only show confirmation when leaving installation option page
    if (from.path === '/Installation/Config' && to.path !== '/Installation/Progress') {
      const confirm = useConfirm()
      try {
        await new Promise((resolve, reject) => {
          confirm.require({
            message: 'Are you sure you want to leave? All changes will be lost.',
            group: 'dialog',
            header: 'Confirm',
            icon: 'mir warning',
            rejectProps: {
              label: 'Cancel',
              severity: 'secondary',
              outlined: true,
              icon: 'mir close',
            },
            acceptProps: {
              label: 'Leave',
              icon: 'mir navigate_next',
            },
            accept: () => resolve(true),
            reject: () => reject(),
          })
        })
      } catch {
        return false
      }
      installationConfig.$reset()
    }

    if (from.path === '/Installation/Progress') {
      installationConfig.$reset()
    }

    return true
  })
}

export function goTo(path: string) {
  router.push(path)
}

export default router
