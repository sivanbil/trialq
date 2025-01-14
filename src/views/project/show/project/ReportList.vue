<template>
  <div>
    <h4 class="text-lg font-semibold mb-2">报告归档</h4>

    <!-- 数据列表 -->
    <div v-if="filteredDataList.length > 0">
      <table class="min-w-full divide-y divide-gray-200 w-full">
        <thead class="bg-gray-50">
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">项目</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">报告编号</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">源文件</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">日期</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
        </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
        <tr v-for="(item, index) in filteredDataList" :key="index">
          <td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500">{{ item.projectName }}</td>
          <td class="px-6 py-4 text-sm text-gray-500">{{ item.reportNumber }}</td>
          <td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 text-left">
            <!-- 遍历 sourceFiles 数组 -->
            <div v-for="(file, fileIndex) in item.sourceFiles" :key="fileIndex" class="block mb-1">
                <span
                    @click="viewData(file)"
                    class="cursor-pointer px-2 py-1 bg-blue-100 text-blue-800 text-xs font-medium rounded-full hover:bg-blue-200"
                >
                  {{ file }}
                </span>
            </div>
          </td>
          <td class="py-4 whitespace-nowrap text-sm text-gray-500">{{ item.stateName }}</td>
          <td class="py-4 whitespace-nowrap text-sm text-gray-500">{{ item.createTime }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
            <button
                v-if="item.state === 2"
                @click="viewItem(item)"
                class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-orange-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              查看
            </button>

            <button
                v-if="item.state !== 2"
                @click="analyzeData(item)"
                class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-orange-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              汇总数据
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
      <div class="mt-1">
        <!-- 分页控件 -->
        <Pagination
            v-if="filteredDataList.length > 0"
            :currentPage="currentPage"
            :totalPages="totalPages"
            @update:currentPage="handlePageChange"
        />

      </div>


    </div>

    <!-- 无数据时的提示 -->
    <div v-else class="text-center py-6 text-gray-500">
      暂无数据，请去导入项目相关的表格进行数据分析。
    </div>

    <!-- 查看数据的 Dialog -->
    <SlotDialog :isOpen="isDialogOpen" :showConfirm="false" title="测试" @close="closeDialog">
      <SummaryView :reportNumber="reportNumber" />
    </SlotDialog>
  </div>
</template>

<script>
import SlotDialog from '@/components/SlotDialog.vue';
import SummaryView from '@/views/project/show/dashboard/SummaryView.vue';
import Pagination from "@/components/PaginationView.vue";

export default {
  name: 'ReportList',
  components: {
    Pagination,
    SlotDialog,
    SummaryView,
  },
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
      isDialogOpen: false, // 控制 Dialog 的显示
      reportNumber: '', // 当前报告号
    };
  },
  computed: {
    // 根据项目号过滤数据
    filteredDataList() {
      return this.dataList;
    },
  },
  watch: {
    // 监听项目号变化，重置页码
    projectNumber() {
      this.currentPage = 1;
      this.fetchData();
    },
    currentPage() {
      this.fetchData();
    },
  },
  mounted() {
    // 组件挂载时获取数据
    this.fetchData();
  },
  methods: {
    // 处理页码变化
    handlePageChange(newPage) {
      this.currentPage = newPage;
      this.fetchData();
    },
    // 获取数据
    async fetchData() {
      try {
        const response = await this.$rustInvoke('fetch_report_list', {
          projectNumber: this.projectNumber,
          currentPage: this.currentPage,
          pageSize: this.pageSize,
        });
        this.dataList = response.data || [];
        this.totalPages = Math.ceil(response.total / this.pageSize);
      } catch (error) {
        console.error('获取报告列表失败:', error);
        this.dataList = [];
        this.totalPages = 1;
      }
    },
    // 查看项目
    viewItem(item) {
      this.reportNumber = item.reportNumber;
      this.isDialogOpen = true; // 打开 Dialog
    },
    // 关闭 Dialog
    closeDialog() {
      this.isDialogOpen = false;
    },
    // 删除项目
    deleteItem(index) {
      if (confirm('确定删除该项目吗？')) {
        // 处理删除逻辑
        this.dataList.splice(index, 1); // 从列表中移除
        this.$emit('delete-item', index); // 触发父组件事件
      }
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
    // 汇总数据
    async analyzeData(item) {
      // 显示不可关闭的提示框
      const closeModal = this.$showModal('系统正在分析数据，请稍候...', {
        showCloseButton: false, // 隐藏关闭按钮
        allowOutsideClick: false, // 禁止点击外部关闭
      });
      try {
        // 调用数据分析接口
        const result = await this.$rustInvoke('analyze_report_data', {
          projectNo: item.projectName,
          reportNo: item.reportNumber,
        });
        if (result.valid) {
          // 这里可以根据返回的数据更新 UI 或显示提示信息
          this.$showModal('数据分析完成！');
        } else {
          this.$showModal('数据分析失败，请重试！');
        }
      } catch (error) {
        this.$showModal('网络异常，请稍后重试！');
      }
      // 关闭提示框
      closeModal();
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>