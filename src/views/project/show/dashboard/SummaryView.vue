<template>
  <div class="bg-gray-100">
    <div class="mx-auto bg-white shadow-md rounded-lg">
      <!-- 数据检查 -->
      <div v-if="hasData">

        <!-- 表格容器 -->
        <SummaryTable :tableData="tableData" :merges="merges" :exportFileNamePrefix="reportNumber" :headers="headers" :totalConfig="totalConfig"/>
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
      headers: {},
      totalConfig: {
        mergeRange: { start: 0, end: 2 }, // 合并第一列到第三列（索引 0 到 2）
        columns: [3, 4, 5, 6,7, 8,9,10,11,12,13,14,15,16], // 统计第四列的总和
        percentageColumns: {
          9: { numerator: 4, denominator: 3 }, // 第 5 列（索引 4）是百分比列，分子为第 4 列（索引 3），分母为第 6 列（索引 5）
          10: { numerator: { subtract: [4, 8] }, denominator: 4 }, // 第 5 列（索引 4）是百分比列，分子为第 4 列（索引 3），分母为第 6 列（索引 5）
        }
      },
      headersMap: [
        {
          site: 'SITE',
          site_name: 'SITE NAME',
          cra: 'CRA',
          pages: 'Pages ',
          pages_entered: 'Pages Entered',
          missing_pages: 'Missing pages',
          md_gt7: 'MP＞7days',
          md_gt14: 'MP>14days',
          sdv_backlog: 'SDV Backlog',
          percent_pages_entered: '% Pages Entered',
          percent_pages_sdved: 'Pages SDVed',
          answered_query: 'Answered Query',
          opened_query: 'Open Query',
          op_gt7: '>7days',
          op_gt14: '>14days',
          op_gt21: '＞21days',
          op_gt30: '≥30days',
          edc_status_comment: 'EDC Status Comment'
        }
      ]
    };
  },
  computed: {

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
          this.headers = this.headersMap[0];
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
