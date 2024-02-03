import {createApp} from "vue"
import "./styles.css"
import App from "./App.vue"
import router from './router'
import {createIcons} from '@/fontawesome.ts'
import ViewUIPlus from 'view-ui-plus'
import 'view-ui-plus/dist/styles/viewuiplus.css'

const app = createApp(App)
createIcons(app)
app.use(router)
app.use(ViewUIPlus)
app.mount("#app")
