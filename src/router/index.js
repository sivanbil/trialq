import { createRouter, createWebHistory } from 'vue-router';
import HomePage from '../views/HomePage.vue'; // 引入 HomePage 组件
import ProjectManagement from '../views/project/ProjectManagement.vue'; // 引入 ProjectManagement 组件
import ToolList from "@/views/tools/ToolList.vue";
import UserSpace from "@/views/user/UserSpace.vue";
import TrialQChat from "@/views/user/LLMChat.vue";
import NotFound from '../views/NotFound.vue'; // 引入 404 组件

const routes = [
    {
        path: '/', // 根路径
        name: 'HomePage',
        component: HomePage, // 默认显示 HomePage
    },
    {
        path: '/project/management', // 项目管理页面
        name: 'ProjectManagement',
        component: ProjectManagement,
    },
    {
        path: '/tool/list', // 工具
        name: 'ToolList',
        component: ToolList,
    },
    {
        path: '/user/space', // 个人设置
        name: 'UserSpace',
        component: UserSpace,
    },
    {
        path: '/trialq/chat',
        name: 'LLMChat',
        component: TrialQChat,
    },
    {
        path: '/:pathMatch(.*)*', // 捕获所有未匹配的路由
        name: 'NotFound',
        component: NotFound,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
