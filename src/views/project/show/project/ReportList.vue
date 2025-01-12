<template>
  <div>
    <h4 class="text-lg font-semibold mb-2">报告归档</h4>

    <!-- 数据列表 -->
    <div v-if="filteredDataList.length > 0">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">项目名称</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">报告编号</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">生成日期</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
        </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
        <tr v-for="(item, index) in filteredDataList" :key="index">
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.projectName }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.reportNumber }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.createTime }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
            <button
                @click="viewItem(item)"
                class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-orange-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              查看
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

      <!-- 分页控件 -->
      <div class="mt-4 flex justify-between items-center">
        <button
            @click="prevPage"
            :disabled="currentPage === 1"
            class="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          上一页
        </button>
        <span class="text-sm text-gray-700">
          第 {{ currentPage }} 页 / 共 {{ totalPages }} 页
        </span>
        <button
            @click="nextPage"
            :disabled="currentPage === totalPages"
            class="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          下一页
        </button>
      </div>
    </div>

    <!-- 无数据时的提示 -->
    <div v-else class="text-center py-6 text-gray-500">
      暂无数据，请去导入项目相关的表格进行数据分析。
    </div>
  </div>
</template>

<script>
export default {
  name: 'ReportList',
  props: {
    projectNumber: {
      type: String,
      default: '', // 默认值为空，表示获取所有项目报告
    },
    pageSize: {
      type: Number,
      default: 10, // 每页显示的数据条数
    },
  },
  data() {
    return {
      dataList: [], // 从后端获取的完整数据
      currentPage: 1, // 当前页码
      totalPages: 1, // 总页数
    };
  },
  computed: {
    // 根据项目号过滤数据
    filteredDataList() {
      let filteredList = this.dataList;
      if (this.projectNumber) {
        filteredList = this.dataList.filter(item => item.projectNumber === this.projectNumber);
      }
      // 分页逻辑
      const startIndex = (this.currentPage - 1) * this.pageSize;
      const endIndex = startIndex + this.pageSize;
      return filteredList.slice(startIndex, endIndex);
    },
  },
  watch: {
    // 监听项目号变化，重置页码
    projectNumber() {
      this.currentPage = 1;
      this.fetchData();
    },
  },
  mounted() {
    // 组件挂载时获取数据
    this.fetchData();
  },
  methods: {
    // 获取数据
    async fetchData() {
      try {
        const response = await this.$rustInvoke('fetch_report_list', {
          projectNumber: this.projectNumber,
        });
        this.dataList = response.data || [];
        this.totalPages = Math.ceil(this.dataList.length / this.pageSize);
      } catch (error) {
        console.error('获取报告列表失败:', error);
        this.dataList = [];
        this.totalPages = 1;
      }
    },
    // 查看项目
    viewItem(item) {
      this.$emit('view-item', item);
    },
    // 删除项目
    deleteItem(index) {
      this.$emit('delete-item', index);
    },
    // 上一页
    prevPage() {
      if (this.currentPage > 1) {
        this.currentPage -= 1;
      }
    },
    // 下一页
    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.currentPage += 1;
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>