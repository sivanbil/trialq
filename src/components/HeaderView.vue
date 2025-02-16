<template>
  <header class="bg-white shadow-md border-b">
    <div class="mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between items-center h-16">
        <!-- 左侧 Logo 或标题 -->
        <div class="flex-shrink-0 left-0 text-left">
          <router-link to="/" class="text-xl font-bold text-purple-800">
            {{ $slogan.main }} <span style="font-size:12px;">{{$slogan.version}}</span>
          </router-link>
          <p class="text-sm text-gray-500 mt-1">{{ $slogan.sub }}</p>
        </div>

        <!-- 右侧菜单 -->
        <nav class="flex space-x-4">
          <router-link
              v-for="(item, index) in menuItems"
              :key="index"
              :to="item.route"
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors flex items-center"
              :class="{
              'bg-purple-800 text-white': isActive(item.route),
              'text-gray-700 hover:bg-purple-100': !isActive(item.route),
            }"
          >
            <component :is="item.icon" class="w-5 h-5 mr-2" />
            {{ item.name }}
          </router-link>
        </nav>
      </div>
    </div>
  </header>
</template>

<script>
import {HomeIcon, Cog6ToothIcon, WrenchScrewdriverIcon, CakeIcon} from '@heroicons/vue/24/outline';

export default {
  name: 'HeaderView',
  data() {
    return {
      // 动态菜单项
      menuItems: [
        { name: '项目管理', route: '/project/management', icon: HomeIcon },
        { name: '常用工具', route: '/tool/list', icon: WrenchScrewdriverIcon },
        { name: '个人设置', route: '/user/space', icon: Cog6ToothIcon },
        { name: 'TrialQChat', route: '/trialq/chat', icon: CakeIcon },

      ],
    };
  },
  methods: {
    // 判断当前路由是否匹配
    isActive(route) {
      return this.$route.path === route;
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>
