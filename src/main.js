import { createApp } from 'vue'
import { createPinia } from 'pinia'
import "@fontsource/cairo/400.css";
import "@fontsource/cairo/600.css";
import "@fontsource/cairo/700.css";
import './style.css'
import App from './App.vue'

const app = createApp(App)
app.use(createPinia())
app.mount('#app')
