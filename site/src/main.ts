import Vue, { createApp } from "vue";
import { plugin as Slicksort } from 'vue-slicksort';
import App from "./App.vue";
import router from "./router";

import '@/theme/fonts.scss'
import '@/theme/colours.scss'
import '@/theme/global.scss'

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faGear, faHouse, faCalendarDays, faSquareCheck } from '@fortawesome/free-solid-svg-icons'

library.add(faGear, faHouse, faCalendarDays, faSquareCheck)

const app = createApp(App);

app
    .use(Slicksort)
    .use(router)
    .component('font-awesome-icon', FontAwesomeIcon)
    .mount("#app");
