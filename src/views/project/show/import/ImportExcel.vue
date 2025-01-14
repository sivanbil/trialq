<template>
  <div class="bg-white p-6 rounded-lg shadow-md text-left">
    <!-- 导入表格按钮 -->
    <div class="mb-6 text-right">
      <button
          @click="openDrawer"
          class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
      >
        导入源数据表格
      </button>
    </div>

    <!-- 数据列表 -->
    <ReportList
        :projectNumber="projectNumber"
        :pageSize="10"
    />

    <!-- 抽屉表单 -->
    <DrawerForm
        :isOpen="isDrawerOpen"
        @close="closeDrawer"
        @confirm-import="handleConfirmImport"
    />
  </div>
</template>

<script>
import DrawerForm from './ImportExcelDrawerForm.vue';
import ReportList from '@/views/project/show/project/ReportList.vue';

export default {
  name: 'DashBoard',
  components: {
    DrawerForm,
    ReportList,
  },
  data() {
    return {
      projectNumber: '', // 当前项目号
      isDrawerOpen: false, // 控制抽屉的显示
    };
  },
  methods: {
    // 打开抽屉
    openDrawer() {
      this.isDrawerOpen = true;
    },
    // 关闭抽屉
    closeDrawer() {
      this.isDrawerOpen = false;
    },
    // 处理确认导入
    handleConfirmImport(data) {
      // 处理导入逻辑
      console.log(data);
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>