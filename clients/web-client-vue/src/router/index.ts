import {route} from "quasar/wrappers";
import {
    createMemoryHistory, createRouter, createWebHashHistory, createWebHistory, Router as VueRouter,
} from "vue-router";
import routes from "./routes";
import {Store} from "src/stores";

/*
 * If not building with SSR mode, you can
 * directly export the Router instantiation;
 *
 * The function below can be async too; either use
 * async/await or return a Promise which resolves
 * with the Router instance.
 */

export let GMIRouter: VueRouter;
export default route(function (/* { store, ssrContext } */) {
    const createHistory = process.env.SERVER ? createMemoryHistory :
        (process.env.VUE_ROUTER_MODE === "history" ? createWebHistory : createWebHashHistory);

    GMIRouter = createRouter({
        scrollBehavior: () => ({
            left: 0,
            top: 0,
        }),
        routes,

        // Leave this as is and make changes in quasar.conf.js instead!
        // quasar.conf.js -> build -> vueRouterMode
        // quasar.conf.js -> build -> publicPath
        history: createHistory(process.env.MODE === "ssr" ? void 0 : process.env.VUE_ROUTER_BASE),
    });

    // Global guard to check authentication.
    GMIRouter.beforeEach((to, from, next) => {
        if (to.matched.some(record => record.meta.requiresAuth)) {
            // Check authentication.
            if (!Store.auth.isLoggedIn) {
                Store.global.redirectedFrom = to.fullPath;

                next({
                    name: "Home",
                });
                return;
            }
        }

        if (to.matched.some(record => record.meta.requiresWallet)) {
            // Check wallet.
            if (!Store.auth.isLoggedIn) {
                Store.global.redirectedFrom = to.fullPath;

                next({
                    name: "Home",
                });
                return;
            }
        }

        next();
    });

    return GMIRouter;
});
