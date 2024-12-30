import { createMemoryHistory, createRouter } from 'vue-router'

import Installation from './Installation.vue'
import Settings from './Settings.vue'
import Installation_Option from './Installation/Option.vue'
import Installation_Progress from './Installation/Progress.vue'

const routes = [
  { path: '/', component: Installation },
  { path: '/Settings', component: Settings },
  { path: '/Installation/Option', component: Installation_Option },
  { path: '/Installation/Progress', component: Installation_Progress },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router
