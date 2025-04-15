import { createRouter, createWebHashHistory } from 'vue-router'
import HomePage from '@/pages/HomePage.vue'
import TestAPage from '@/pages/TestAPage.vue'
import TestCPage from '@/pages/TestCPage.vue'
import TestBPage from '@/pages/TestBPage.vue'
import { useMessagesStore } from '@/c-lib/stores/messages'

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomePage,
        },
        {
            path: '/testa',
            name: 'testa',
            component: TestAPage,
        },
        {
            path: '/testb',
            name: 'testb',
            component: TestBPage,
        },
        {
            path: '/testc',
            name: 'testc',
            component: TestCPage,
        },
    ],
})

router.beforeEach(() => {
    const messages = useMessagesStore()
    messages.remove('route')
})

export default router
