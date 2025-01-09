<template>
  <div class="bg-white p-6 rounded-lg shadow-md text-left">
    <!-- 导入表格按钮 -->
    <div class="mb-6 text-right">
      <button
          @click="openDrawer"
          class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
      >
        导入表格
      </button>
    </div>

    <!-- 数据列表 -->
    <div>
      <h4 class="text-lg font-semibold mb-2">数据归档</h4>
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">日期</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">项目编号</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
        </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
        <tr v-for="(item, index) in dataList" :key="index">
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.date }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.projectId }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
            <button
                @click="viewItem(index)"
                class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-orange-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              查看
            </button>
            <button
                @click="exportItem(index)"
                class="px-3 py-1 ml-2 text-sm font-medium text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              导出
            </button>
            <button
                @click="deleteItem(index)"
                class="ml-2 px-3 py-1 text-sm font-medium text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500"
            >
              删除
            </button>
          </td>
        </tr>
        </tbody>
      </table>
    </div>

    <!-- 抽屉表单 -->
    <DrawerForm
        :isOpen="isDrawerOpen"
        @close="closeDrawer"
        @confirm-import="handleConfirmImport"
    />
  </div>
</template>

<script>
import DrawerForm from './DashBoardDrawerForm.vue'; // 引入 DrawerForm 组件

export default {
  name: 'DashBoard',
  components: {
    DrawerForm, // 注册 DrawerForm 组件
  },
  data() {
    return {
      // 数据列表
      dataList: [
        { date: '2023-10-01', projectId: 'P001' },
        { date: '2023-10-02', projectId: 'P002' },
        { date: '2023-10-03', projectId: 'P003' },
      ],
      // 抽屉表单相关状态
      isDrawerOpen: false,
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
      this.dataList = [...this.dataList, ...data];
    },
    // 查看项目
    viewItem(index) {
      const item = this.dataList[index];
      alert(`查看项目：${item.projectId}`);
    },
    // 导出项目
    exportItem(index) {
      const item = this.dataList[index];
      alert(`导出项目：${item.projectId}`);
    },
    // 删除项目
    deleteItem(index) {
      if (confirm('确定删除该项目吗？')) {
        this.dataList.splice(index, 1);
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>