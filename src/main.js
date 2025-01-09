import { createApp } from 'vue'
import App from './App.vue'
import router from './router'; // 引入路由配置

const app = createApp(App);

import BackButton from './components/BackButton.vue'; // 引入返回组件
import CoolBanner from './components/CoolBanner.vue'; // 引入Banner组件
import HeaderView from '@/components/HeaderView.vue';
import FooterView from './components/FooterView.vue';


app.component('FooterView', FooterView);

app.component('BackButton', BackButton);

app.component('CoolBanner', CoolBanner);

app.component('HeaderView', HeaderView);


import '@/assets/tailwind.css';


import './assets/main.css'; // 引入全局样式


import sloganPlugin from './plugins/sloganPlugin';

//
const { invoke } = window.__TAURI__.core;
app.config.globalProperties.$rustInvoke = invoke;

app.use(sloganPlugin);

app.use(router).mount('#app')
