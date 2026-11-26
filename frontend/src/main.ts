import { createPinia } from 'pinia'
import { createApp } from 'vue'

import App from './App.vue'
import './assets/main.css'
import { APP_NAME } from './constants'
import router from './router'

document.title = APP_NAME

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
