import { createMemoryHistory, createRouter } from "vue-router";

import Installation from "@/Installation.vue";
import Installation_Option from "@/Installation/Option.vue";
import Installation_Progress from "@/Installation/Progress.vue";
import Settings from "@/Settings.vue";

const routes = [
  { path: "/Installation", component: Installation },
  { path: "/Installation/Option", component: Installation_Option },
  { path: "/Installation/Progress", component: Installation_Progress },
  { path: "/Settings", component: Settings },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

router.push("/Installation");

export function goTo(path: string) {
  router.push(path);
}

export default router;
