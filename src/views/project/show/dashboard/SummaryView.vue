<template>
  <div class="bg-gray-100 p-6">
    <div class="mx-auto bg-white shadow-md rounded-lg p-6">
      <!-- 数据检查 -->
      <div v-if="hasData">
        <!-- 按钮组 -->
        <div class="flex justify-between items-center mb-6">
          <button
              @click="exportTable"
              class="btn-success"
          >
            导出表格
          </button>
        </div>
        <!-- 表格容器 -->
        <SummaryTable :tableData="tableData" :merges="merges" />
      </div>
      <div v-else>
        该报告已作废，没有任何数据
      </div>
    </div>
  </div>
</template>

<script>
import * as XLSX from 'xlsx';
import SummaryTable from './SummaryTable.vue'; // 引入表格组件

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
    // 检查是否有数据
    hasData() {
      return (
          Array.isArray(this.tableData) &&
          this.tableData.length > 0 &&
          Array.isArray(this.merges) &&
          this.merges.length > 0
      );
    },
    // 计算列总和
    columnTotals() {
      if (!this.hasData) return []; // 如果没有数据，返回空数组

      const totals = [];
      for (let c = 1; c < this.tableData[0].length; c++) {
        let total = 0;
        for (let r = 2; r < this.tableData.length; r++) {
          const value = parseFloat(this.tableData[r][c]);
          if (!isNaN(value)) {
            total += value;
          }
        }
        totals.push(total);
      }
      return totals;
    },
  },
  methods: {
    // 导出表格
    exportTable() {
      if (!this.hasData) {
        alert('没有数据可导出');
        return;
      }

      const table = document.querySelector('table');
      const ws = XLSX.utils.table_to_sheet(table);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, 'Sheet1');
      XLSX.writeFile(wb, 'exported_table.xlsx');
    },
    // 返回工作空间
    goToWorkspace() {
      window.location.href = '/workspace.html';
    },
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
  },
  mounted() {
    // 组件挂载时计算列总和
    this.$nextTick(() => {
      this.columnTotals;
    });
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