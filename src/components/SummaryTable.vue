<template>
  <!-- 按钮组 -->
  <div class="flex justify-between items-center mb-6">
    <button
        @click="exportTable"
        class="btn-success"
    >
      导出表格
    </button>
  </div>
  <div id="tableContainer" class="overflow-x-auto overflow-y-auto max-h-[500px]">
    <table class="min-w-full bg-white border border-gray-200">
      <thead>
      <tr>
        <!-- 渲染表头 -->
        <th
            v-for="(header, index) in headers"
            :key="index"
            class="px-6 py-4 whitespace-nowrap text-sm font-semibold text-gray-900 border border-gray-200"
        >
          {{ header }}
        </th>
      </tr>
      </thead>
      <tbody>
      <!-- 渲染数据行 -->
      <tr v-for="(row, rowIndex) in tableData" :key="rowIndex">
        <td
            v-for="(header, cellIndex) in headers"
            :key="cellIndex"
            class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 border border-gray-200"
        >
          {{ row[header] }} <!-- 根据表头提取数据 -->
        </td>
      </tr>
      <!-- 如果没有数据，显示提示 -->
      <tr v-if="!hasData">
        <td
            colspan="100%"
            class="px-6 py-4 whitespace-nowrap text-sm text-center text-gray-500 border border-gray-200"
        >
          暂无数据
        </td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import * as XLSX from 'xlsx';

export default {
  name: 'SummaryTable',
  props: {
    tableData: {
      type: Array,
      required: true,
    },
    merges: {
      type: Array,
      required: true,
    },
    exportFileNamePrefix: {
      type: String,
      default: '导出表格', // 默认导出文件名称前缀
    },
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
    // 生成文件名
    generateFileName() {
      const now = new Date();
      const year = now.getFullYear();
      const month = String(now.getMonth() + 1).padStart(2, '0'); // 月份补零
      const day = String(now.getDate()).padStart(2, '0'); // 日期补零
      const hours = String(now.getHours()).padStart(2, '0'); // 小时补零
      const minutes = String(now.getMinutes()).padStart(2, '0'); // 分钟补零
      const seconds = String(now.getSeconds()).padStart(2, '0'); // 秒补零

      // 格式：指定名称 + 年月日时分秒
      return `${this.exportFileNamePrefix}${year}${month}${day}${hours}${minutes}${seconds}.xlsx`;
    },
    // 导出表格
    exportTable() {
      if (!this.hasData) {
        alert('没有数据可导出');
        return;
      }

      // 创建表头和数据
      const headers = this.headers; // 表头
      const data = this.tableData.map(row => headers.map(header => row[header])); // 数据行

      // 将表头和数据合并为一个二维数组
      const sheetData = [headers, ...data];

      // 创建工作表
      const ws = XLSX.utils.aoa_to_sheet(sheetData);

      // 创建工作簿并添加工作表
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, 'Sheet1');

      // 生成文件名
      const fileName = this.generateFileName();

      // 导出文件
      XLSX.writeFile(wb, fileName);
    },
    // 获取单元格的 colspan
    getColSpan(rowIndex, cellIndex) {
      if (!this.merges || !this.merges.length) return 1;

      for (const merge of this.merges) {
        if (merge.s.r === rowIndex && merge.s.c === cellIndex) {
          return merge.e.c - merge.s.c + 1;
        }
      }
      return 1;
    },
  },
};
</script>

<style scoped>
#tableContainer {
  max-height: 500px;
  overflow-y: auto;
  overflow-x: auto;
}
</style>