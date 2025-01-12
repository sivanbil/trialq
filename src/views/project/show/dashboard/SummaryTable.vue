<template>
  <div id="tableContainer" class="overflow-x-auto">
    <table class="min-w-full bg-white border border-gray-200">
      <thead>
      <tr v-for="(row, rowIndex) in tableData" :key="rowIndex">
        <td
            v-for="(cell, cellIndex) in row"
            :key="cellIndex"
            :colspan="getColSpan(rowIndex, cellIndex)"
            :class="[
              'px-6 py-4 whitespace-nowrap text-sm border border-gray-200',
              rowIndex === 0 ? 'font-semibold text-gray-900' : 'text-gray-900',
            ]"
        >
          {{ cell }}
        </td>
      </tr>
      </thead>
      <tbody>
      <tr v-if="hasData">
        <td
            class="px-6 py-4 whitespace-nowrap text-sm font-semibold text-gray-900 border border-gray-200"
        >
          Total
        </td>
        <td
            v-for="(total, index) in columnTotals"
            :key="index"
            class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 border border-gray-200"
        >
          {{ total }}
        </td>
      </tr>
      <tr v-else>
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
  },
  computed: {
    // 检查是否有数据
    hasData() {
      return this.tableData && this.tableData.length > 0;
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
    // 获取单元格的 colspan
    getColSpan(rowIndex, cellIndex) {
      if (!this.merges || !this.merges.length) return 1; // 如果没有合并配置，返回默认值

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
/* 如果需要自定义样式，可以在这里添加 */
</style>