import { createMemoryHistory, createRouter } from 'vue-router'

import Start from './Start.vue'
import Settings from './Settings.vue'
import Installation_Config from './Installation/Config.vue'
import Installation_Progress from './Installation/Progress.vue'

const routes = [
  { path: '/', component: Start },
  { path: '/Settings', component: Settings },
  { path: '/Installation/Config', component: Installation_Config },
  { path: '/Installation/Progress', component: Installation_Progress },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router
