<template>
  <!-- 按钮组 -->
  <div class="mb-6 text-right">
    <button @click="exportTable" class="btn-success bg-blue-500 text-white px-2 py-2 rounded">
      导出表格
    </button>
  </div>
  <div id="tableContainer" class="overflow-x-auto overflow-y-auto max-h-[500px]" style="zoom:0.5">
    <table id="exportTable" class="min-w-full bg-white border border-gray-200">
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
      <tr v-for="(row, rowIndex) in tableData" :key="generateRowKey(row, rowIndex)">
        <td
            v-for="(_, cellIndex) in headers"
            :key="cellIndex"
            class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 border border-gray-200"
        >
          {{ row[cellIndex] }} <!-- 根据表头提取数据 -->
        </td>
      </tr>

      <!-- 如果有需要统计的列，显示汇总行 -->
      <tr v-if="hasData && totalConfig.columns.length > 0" class="bg-orange-500 text-white">
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
import ExcelJS from 'exceljs';

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
        let numeratorTotal;
        if (typeof numerator === 'number') {
          numeratorTotal = totals[numerator];
        } else if (numerator && numerator.subtract) {
          const [column1, column2] = numerator.subtract;
          const total1 = totals[column1];
          const total2 = totals[column2];
          if (total1!== undefined && total2!== undefined) {
            numeratorTotal = total1 - total2;
          }
        }
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
    // 生成行的唯一 key
    generateRowKey(row, rowIndex) {
      // 这里假设数据中有一个唯一标识字段，如果没有可以用 rowIndex 作为备用
      return row.id || rowIndex;
    },
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
    async exportTable() {
      if (!this.hasData) {
        alert('没有数据可导出');
        return;
      }

      const workbook = new ExcelJS.Workbook();
      const worksheet = workbook.addWorksheet('Sheet1');

      const headerKeys = Object.keys(this.headers);
      // 添加表头
      worksheet.addRow(headerKeys.map(key => this.headers[key]));

      // 添加数据行
      this.tableData.forEach((row) => {
        const rowData = headerKeys.map(key => row[key]);
        worksheet.addRow(rowData);
      });

      // 如果有需要统计的列，添加汇总行
      if (this.hasData && this.totalConfig.columns.length > 0) {
        const summaryRow = [];
        const startCol = this.totalConfig.mergeRange.start;
        const endCol = this.totalConfig.mergeRange.end;
        for (let i = 0; i < startCol; i++) {
          summaryRow.push('');
        }
        summaryRow.push('Grand Total');
        for (let i = startCol + 1; i <= endCol; i++) {
          summaryRow.push('');
        }
        Object.entries(this.headers).slice(endCol + 1).forEach(([/* _ */, /* header */], cellIndex) => {
          const colIndex = cellIndex + endCol + 1;
          summaryRow.push(this.totalConfig.columns.includes(colIndex) ? this.totals[colIndex] : '');
        });
        const summaryRowNumber = this.tableData.length + 1;

        worksheet.addRow(summaryRow);

        // 合并Grand Total行前三列单元格
        worksheet.mergeCells(summaryRowNumber+1, 1, summaryRowNumber+1, 3);

        // 设置合并单元格背景颜色为橙色
        const mergedCell = worksheet.getCell(summaryRowNumber+1, 1);
        mergedCell.fill = {
          type: 'pattern',
          pattern: 'solid',
          fgColor: {argb: 'FFFFA500'}
        };

        // 设置Grand Total行其他有数据的单元格背景色为橙色
        Object.entries(this.headers).slice(endCol + 1).forEach(([/* _ */, /* header */], cellIndex) => {
          const colIndex = cellIndex + endCol + 1;
          if (this.totalConfig.columns.includes(colIndex)) {
            const cell = worksheet.getCell(summaryRowNumber+1, colIndex+1);
            cell.font = {
              color: {argb: 'FFFFA500'}
            };
            cell.fill = {
              type: 'pattern',
              pattern: 'solid',
              fgColor: {argb: 'FFFFA500'},
            };
          }
        });
      }

      // 设置表头字体样式（加粗）
      const headerFontStyle = {
        name: 'Arial',
        size: 11,
        bold: true
      };
      const normalFontStyle = {
        name: 'Arial',
        size: 11
      };

      worksheet.eachRow((row, rowNumber) => {
        row.eachCell((cell) => {
          if (rowNumber === 1) {
            cell.font = headerFontStyle;
          } else if (rowNumber === this.tableData.length + 1) {
            // 设置汇总行字体加粗
            cell.font = {...normalFontStyle, bold: true};
          } else {
            cell.font = normalFontStyle;
          }
          cell.alignment = {horizontal: 'left'}; // 设置左对齐
        });
      });

      // 生成文件名
      const fileName = this.generateFileName();
      // 保存文件
      const buffer = await workbook.xlsx.writeBuffer();
      const blob = new Blob([buffer], {type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'});
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = fileName;
      a.click();
      window.URL.revokeObjectURL(url);
    }
  }
};
</script>

<style scoped>
#tableContainer {
  max-height: 500px;
  overflow-y: auto;
  overflow-x: auto;
}
</style>
