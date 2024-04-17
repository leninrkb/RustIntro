import { createMemoryHistory, createRouter } from 'vue-router'
import HomeView from '../views/HomeView.vue';

const routes = [
    {
        path: '/',
        name:'home-view',
        component: HomeView
    }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router