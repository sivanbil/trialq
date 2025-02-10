<template>
  <!-- 按钮组 -->
  <div class=" mb-6 text-right">
    <button
        @click="exportTable"
        class="btn-success bg-blue-500 text-white px-2 py-2 rounded"
    >
      导出表格
    </button>
  </div>
  <div id="tableContainer" class="overflow-x-auto overflow-y-auto max-h-[500px]" style="zoom:0.5">
    <table  id="exportTable" class="min-w-full bg-white border border-gray-200">
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
            v-for="(_, cellIndex) in headers"
            :key="cellIndex"
            class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 border border-gray-200"
        >
          {{ row[cellIndex] }} <!-- 根据表头提取数据 -->
        </td>
      </tr>

      <!-- 如果有需要统计的列，显示汇总行 -->
      <tr v-if="hasData && totalConfig.columns.length > 0" class="bg-blue-500 text-white">
        <td
            :colspan="totalConfig.mergeRange.end - totalConfig.mergeRange.start + 1"
            class="px-6 py-4 whitespace-nowrap text-sm  border border-gray-200 text-center"
        >
          Grand Total
        </td>
        <td
            v-for="(header, cellIndex) in Object.entries(headers).slice(totalConfig.mergeRange.end + 1)"
            :key="cellIndex + totalConfig.mergeRange.end + 1"
            class="px-6 py-4 whitespace-nowrap text-sm  border border-gray-200"
        >
          {{ totalConfig.columns.includes(cellIndex + totalConfig.mergeRange.end + 1) ? totals[cellIndex + totalConfig.mergeRange.end + 1] : '' }}
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
    headers: {
      type: Object,
      required: true,
    },
    exportFileNamePrefix: {
      type: String,
      default: '导出表格', // 默认导出文件名称前缀
    },
    totalConfig: {
      type: Object,
      default: () => ({
        mergeRange: { start: 0, end: 0 }, // 合并单元格的范围，索引从 0 开始
        columns: [], // 需要统计的列索引数组
        percentageColumns: {} // 百分比列的计算规则，格式: { 列索引: { numerator: 分子列索引, denominator: 分母列索引 } }
      })
    }
  },
  computed: {
    // 检查是否有数据
    hasData() {
      return this.tableData && this.tableData.length > 0;
    },
    // 计算汇总数据
    totals() {
      const totals = {};
      const headerKeys = Object.keys(this.headers);
      // 先计算普通列的总和
      this.totalConfig.columns.forEach((colIndex) => {
        if (!this.totalConfig.percentageColumns[colIndex]) {
          const key = headerKeys[colIndex];
          let hasDecimal = false;
          const sum = this.tableData.reduce((acc, row) => {
            const value = parseFloat(row[key]);
            if (!isNaN(value) && Number.isFinite(value)) {
              if (!Number.isInteger(value)) {
                hasDecimal = true;
              }
              return acc + value;
            }
            return acc;
          }, 0);
          totals[colIndex] = hasDecimal ? parseFloat(sum.toFixed(1)) : Math.round(sum);
        }
      });
      // 计算百分比列
      Object.entries(this.totalConfig.percentageColumns).forEach(([colIndexStr, { numerator, denominator }]) => {
        const colIndex = parseInt(colIndexStr);
        const numeratorTotal = totals[numerator];
        const denominatorTotal = totals[denominator];
        if (numeratorTotal!== undefined && denominatorTotal!== undefined && denominatorTotal!== 0) {
          const percentage = (numeratorTotal / denominatorTotal) * 100;
          totals[colIndex] = parseFloat(percentage.toFixed(1));
        }
      });
      return totals;
    }
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

      // 获取表格元素
      const table = document.getElementById('exportTable');
      // 将表格转换为工作表
      const ws = XLSX.utils.table_to_sheet(table);
      // 添加合并单元格信息
      const lastRowIndex = this.tableData.length;
      const startCol = this.totalConfig.mergeRange.start;
      const endCol = this.totalConfig.mergeRange.end;
      ws['!merges'] = ws['!merges'] || [];
      ws['!merges'].push({ s: { r: lastRowIndex, c: startCol }, e: { r: lastRowIndex, c: endCol } });
      // 设置列宽
      this.setColumnWidths(ws, table);
      // 创建工作簿并添加工作表
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, 'Sheet1');
      // 生成文件名
      const fileName = this.generateFileName();
      // 导出文件
      XLSX.writeFile(wb, fileName);
    },
    // 设置列宽
    setColumnWidths(ws, table) {
      // 获取表格的列元素
      const cols = table.querySelectorAll('th, td');
      const colWidths = {};

      // 遍历列元素，计算每列的最大宽度
      cols.forEach((col) => {
        const colIndex = col.cellIndex; // 列索引
        const width = col.offsetWidth; // 列的实际宽度

        // 如果当前列的宽度大于已记录的最大宽度，则更新
        if (!colWidths[colIndex] || width > colWidths[colIndex]) {
          colWidths[colIndex] = width;
        }
      });

      // 设置工作表的列宽
      ws['!cols'] = Object.keys(colWidths).map((colIndex) => ({
        wch: colWidths[colIndex] / 7, // 将像素宽度转换为 Excel 列宽单位
      }));
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