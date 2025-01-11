<template>
  <div
      v-if="isVisible"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center"
      @click.self="close"
  >
    <!-- 弹框容器 -->
    <div class="bg-white p-6 rounded-lg w-11/12 max-w-4xl max-h-[80vh] flex flex-col">
      <!-- 头部区域 -->
      <div class="flex justify-between items-center mb-4">
        <!-- 标题 -->
        <h3 class="text-xl font-semibold text-gray-800">文件预览</h3>
        <!-- 右上角关闭按钮 -->
        <button
            @click="close"
            class="p-2 text-gray-500 hover:text-gray-700 focus:outline-none"
        >
          <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
          >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <!-- 表格内容区域 -->
      <div class="overflow-auto flex-1">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50 sticky top-0">
          <tr>
            <th
                v-for="(header, index) in fileData.headers"
                :key="index"
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            >
              {{ header }}
            </th>
          </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="(row, rowIndex) in fileData.rows" :key="rowIndex">
            <td
                v-for="(cell, cellIndex) in row"
                :key="cellIndex"
                class="px-6 py-4 whitespace-nowrap text-sm text-gray-500"
            >
              {{ cell }}
            </td>
          </tr>
          </tbody>
        </table>
      </div>

      <!-- 底部关闭按钮 -->
      <button
          @click="close"
          class="mt-4 px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
      >
        关闭
      </button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'FilePreviewModal',
  props: {
    isVisible: {
      type: Boolean,
      required: true,
    },
    fileData: {
      type: Object,
      required: true,
    },
  },
  methods: {
    close() {
      this.$emit('close');
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>