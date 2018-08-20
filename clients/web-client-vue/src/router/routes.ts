import {RouteRecordRaw} from "vue-router";

const routes: RouteRecordRaw[] = [{
    path: "/",
    component: () => import("layouts/MainLayout.vue"),
    redirect: {
        name: "Home",
    },
    children: [{
        path: "",
        name: "Home",
        component: () => import("pages/Home.vue"),
    }, {
        path: "/challenges",
        name: "Challenges",
        component: () => import("pages/Challenges.vue"),
    }, {
        path: "/challenge/:challenge",
        name: "Challenge",
        component: () => import("pages/Challenge.vue"),
    }, {
        path: "/challenge/:challenge/:bet",
        name: "Challenge",
        component: () => import("pages/Bet.vue"),
        meta: {
            requiresWallet: true,
            requiresAuth: true,
        },
    }, {
        path: "/faqs",
        name: "Faqs",
        component: () => import("pages/Faqs.vue"),
    }, {
        path: "/about",
        name: "About",
        component: () => import("pages/About.vue"),
    }, {
        path: "/project",
        name: "Project",
        component: () => import("pages/Project.vue"),
    }, {
        path: "/blog",
        name: "Blog",
        component: () => import("pages/Blog.vue"),
    }],
},

    // Always leave this as last one,
    // but you can also remove it
    {
        path: "/:catchAll(.*)*",
        component: () => import("pages/Error404.vue"),
    }];

export default routes;
