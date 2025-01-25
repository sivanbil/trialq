<template>
  <div class="bg-gray-100 p-6">
    <div class="mx-auto bg-white shadow-md rounded-lg p-6">
      <!-- 数据检查 -->
      <div v-if="hasData">

        <!-- 表格容器 -->
        <SummaryTable :tableData="tableData" :merges="merges" :exportFileNamePrefix="reportNumber" />
      </div>
      <div v-else>
        没有任何数据
      </div>
    </div>
  </div>
</template>

<script>
import SummaryTable from '@/components/SummaryTable.vue'; // 引入表格组件

export default {
  name: 'SummaryView',
  components: {
    SummaryTable,
  },
  props: {
    reportNumber: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      // 表格数据
      tableData: [],
      // 合并单元格的配置
      merges: [],
      // 抽屉表单相关状态
      isDrawerOpen: false,
    };
  },
  computed: {
    // 提取表头
    headers() {
      if (!this.tableData || this.tableData.length === 0) return [];
      return Object.keys(this.tableData[0]); // 获取第一个对象的键作为表头
    },
    // 检查是否有数据
    hasData() {
      return this.tableData && this.tableData.length > 0;
    },
  },
  methods: {

    // 获取单元格的 colspan
    getColSpan(rowIndex, cellIndex) {
      if (!Array.isArray(this.merges) || this.merges.length === 0) return 1; // 如果没有合并配置，返回默认值

      for (const merge of this.merges) {
        if (merge.s.r === rowIndex && merge.s.c === cellIndex) {
          return merge.e.c - merge.s.c + 1;
        }
      }
      return 1;
    },
    // 获取报告详情
    async fetchReportDetail() {
      try {
        const response = await this.$rustInvoke("fetch_report_detail", {reportNumber: this.reportNumber});
        if (response && response.valid) {
          this.tableData = response.reportData;
        } else {
          console.error('No data returned from fetch_report_detail');
        }
      } catch (error) {
        console.error('Error fetching report detail:', error);
      }
    },
  },
  mounted() {
    // 组件挂载时获取报告详情
    this.fetchReportDetail();
  },
};
</script>

<style scoped>
.btn-primary {
  @apply px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition duration-200;
}

.btn-success {
  @apply px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition duration-200;
}
</style>