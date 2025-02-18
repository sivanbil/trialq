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
app.config.globalProperties.$showModal = function (message, options = {},title = '提示') {
    // 默认选项
    const {
        showCloseButton = true,
        useListener = false,
        eventName = null,
    } = options;
    // 创建一个临时 div 来挂载模态框
    const modalContainer = document.createElement('div');
    modalContainer.id = "modal-l";
    document.body.appendChild(modalContainer);

    // 创建模态框实例
    const modalInstance = createApp(ModalContainer, {
        visible: true,
        title: title,
        message: message,
        showCloseButton: showCloseButton,
        onClose: () => {
            // 关闭模态框时销毁实例并移除 DOM
            modalInstance.unmount();
            document.body.removeChild(modalContainer);
        },
    });

    // 挂载模态框
    modalInstance.mount(modalContainer);

    // 如果需要监听事件
    if (useListener && eventName) {
        console.debug('listen..........', eventName)
        listen(eventName, (event) => {
            // 在这里处理事件
            console.debug(`Received event: ${eventName}`, event.payload);
            // 根据 id 获取模态框元素并更新内容
            const modalElement = document.getElementById(modalContainer.id);
            if (modalElement) {
                // 假设 ModalContainer 组件中有一个显示消息的元素，这里简单更新消息
                const messageElement = modalElement.querySelector('.modal-message-extra');
                console.debug(messageElement, messageElement.textContent)
                if (messageElement) {
                    messageElement.textContent = `进度情况: ${event.payload}`;
                }
            }
        }).catch((error) => {
            console.error(`Failed to listen to event ${eventName}:`, error);
        });
    }

    // 返回一个 close 函数，允许外部关闭模态框
    return () => {
        modalInstance.unmount();
        document.body.removeChild(modalContainer);
    };
};


import '@/assets/tailwind.css';


import './assets/main.css'; // 引入全局样式


import sloganPlugin from './plugins/sloganPlugin';
console.log(window.__TAURI__)

//
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
app.config.globalProperties.$rustInvoke = invoke;
app.config.globalProperties.$rustListen = listen;

app.use(sloganPlugin);

app.use(router).mount('#app')
