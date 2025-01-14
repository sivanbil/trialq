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
        @view-item="viewItem"
        @delete-item="deleteItem"
    />

    <!-- 抽屉表单 -->
    <DrawerForm
        :isOpen="isDrawerOpen"
        @close="closeDrawer"
        @confirm-import="handleConfirmImport"
    />

    <!-- 查看数据的 Dialog -->
    <SlotDialog :isOpen="isDialogOpen" :showConfirm="false" title="测试" @close="closeDialog">
      <SummaryView :reportNumber="reportNumber" />
    </SlotDialog>


  </div>
</template>

<script>
import DrawerForm from './ImportExcelDrawerForm.vue';
import SlotDialog from '@/components/SlotDialog.vue';
import SummaryView from '@/views/project/show/dashboard/SummaryView.vue';
import ReportList from '@/views/project/show/project/ReportList.vue';

export default {
  name: 'DashBoard',
  components: {
    SummaryView,
    SlotDialog,
    DrawerForm,
    ReportList, // 注册 ReportList 组件
  },
  data() {
    return {
      projectNumber: '', // 当前项目号
      reportNumber: '', // 当前报告号
      // 抽屉表单相关状态
      isDrawerOpen: false,
      isDialogOpen: false,
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
      console.log(data)
    },
    // 查看项目
    viewItem(item) {
      this.reportNumber = item.reportNumber;
      this.openDialog();
      console.log(`查看项目：${item.projectId}`);
    },
    openDialog() {
      this.isDialogOpen = true;
    },
    // 关闭对话框
    closeDialog() {
      this.isDialogOpen = false;
    },
    // 删除项目
    deleteItem(index) {
      console.log(index);
      if (confirm('确定删除该项目吗？')) {
        // 处理删除逻辑
      }
    },

  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>