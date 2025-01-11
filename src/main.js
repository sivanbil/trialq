import { createApp } from 'vue'
import App from './App.vue'
import router from './router'; // 引入路由配置

const app = createApp(App);

import BackButton from './components/BackButton.vue'; // 引入返回组件
import CoolBanner from './components/CoolBanner.vue'; // 引入Banner组件
import HeaderView from '@/components/HeaderView.vue';
import FooterView from './components/FooterView.vue';
import ModalContainer from './components/ModalCommon.vue';


app.component('FooterView', FooterView);

app.component('BackButton', BackButton);

app.component('CoolBanner', CoolBanner);

app.component('HeaderView', HeaderView);

// 注册全局模态框组件
app.component('GlobalModal', ModalContainer);

// 定义全局函数
app.config.globalProperties.$showModal = function (title, message) {
    // 创建一个临时 div 来挂载模态框
    const modalContainer = document.createElement('div');
    document.body.appendChild(modalContainer);

    // 创建模态框实例
    const modalInstance = createApp(ModalContainer, {
        visible: true,
        title: title,
        message: message,
        onClose: () => {
            // 关闭模态框时销毁实例并移除 DOM
            modalInstance.unmount();
            document.body.removeChild(modalContainer);
        },
    });

    // 挂载模态框
    modalInstance.mount(modalContainer);
};


import '@/assets/tailwind.css';


import './assets/main.css'; // 引入全局样式


import sloganPlugin from './plugins/sloganPlugin';

//
const { invoke } = window.__TAURI__.core;

console.log(window.__TAURI__)
app.config.globalProperties.$rustInvoke = invoke;

app.use(sloganPlugin);

app.use(router).mount('#app')
