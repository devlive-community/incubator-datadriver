import {createRouter, createWebHistory, RouteRecordRaw} from 'vue-router'
import LayoutContainer from '@/layout/LayoutContainer.vue'

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "home",
        redirect: "/index",
        component: LayoutContainer,
        children: [{
            path: "index",
            component: () => import("@/views/HomeView.vue")
        }]
    },
    {
        path: '/connector',
        name: 'connector',
        redirect: '/connector/home',
        component: LayoutContainer,
        children: [{
            path: "home",
            component: () => import("@/views/connector/HomeView.vue")
        }]
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router
