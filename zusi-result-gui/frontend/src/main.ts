import './c-lib/assets/tailwind/index.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import i18n from './locale/index'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia())
app.use(i18n)
app.use(router)

app.mount('#app')
