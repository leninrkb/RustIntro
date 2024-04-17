import { createMemoryHistory, createRouter } from 'vue-router'
import HomeView from '../views/HomeView.vue';
import ScopeView from '../views/ScopeView.vue';
import ResultView from '../views/ResultView.vue';

const routes = [
    {
        path: '/home',
        name:'home-view',
        component: HomeView
    },
    {
        path: '/',
        name: 'scope-view',
        component: ScopeView,
    },
    {
        path: '/result',
        name: 'result-view',
        component: ResultView,
    }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router