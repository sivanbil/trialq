<template>
  <div class="min-h-screen bg-gray-50">
    <HeaderView />
    <div class="mx-auto p-4">
      <!-- 标签页导航 -->
      <div class="flex space-x-4 mb-6 border-b border-gray-200">
        <button
            v-for="(tab, index) in tabs"
            :key="index"
            @click="activeTab = index"
            class="px-4 py-2 text-sm font-medium focus:outline-none"
            :class="{
            'text-purple-800 border-b-2 border-purple-800': activeTab === index,
            'text-gray-500 hover:text-gray-700': activeTab !== index,
          }"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- 标签页内容 -->
      <div>
        <!-- 第二个标签页：项目管理 -->
        <div v-if="activeTab === 0">
         <ProjectList/>
        </div>
        <!-- 第一个标签页：数据工作平台和更多管理 -->
        <div v-if="activeTab === 1">
          <!-- DashBoard 组件 -->
          <ImportExcel />

        </div>

        <!-- 第三个标签页：其他 -->
        <div v-if="activeTab === 2" class="p-6 bg-purple-700 text-white rounded-lg shadow-md relative ">
          <!-- 遮盖层 -->
          <div class="absolute inset-0  bg-opacity-50 rounded-lg"></div>
          <p class="text-white">
            <span class="mr-2">📢</span>
            请持续关注 TrialQ 的新版本发布！
          </p>
        </div>
      </div>
    </div>
    <FooterView />
  </div>
</template>

<script>
import ImportExcel from '@/views/project/show/import/ImportExcel.vue'; // 导入 DashBoard 组件
import ProjectList from "./show/project/ProjectList.vue";

export default {
  name: 'ProjectManagement',
  components: {
    ProjectList,
    ImportExcel, // 注册 DashBoard 组件
  },
  data() {
    return {
      // 标签页数据
      tabs: [
        { label: '项目数据维护' },
        { label: 'Excel工作台' },
        { label: '更多' },
      ],
      // 当前激活的标签页索引
      activeTab: 0,
    };
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>