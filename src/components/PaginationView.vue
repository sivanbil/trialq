<template>
  <div class="flex justify-center items-center space-x-2">
    <!-- 上一页按钮 -->
    <button
        @click="prevPage"
        :disabled="currentPage === 1"
        class="px-4 py-2 text-sm font-medium text-purple-800 bg-purple-100 rounded-md hover:bg-purple-200 focus:outline-none focus:ring-2 focus:ring-purple-500 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      上一页
    </button>

    <!-- 当前页码和总页数 -->
    <span class="text-sm text-gray-600">
      第 {{ currentPage }} 页 / 共 {{ totalPages }} 页
    </span>

    <!-- 下一页按钮 -->
    <button
        @click="nextPage"
        :disabled="currentPage === totalPages"
        class="px-4 py-2 text-sm font-medium text-purple-800 bg-purple-100 rounded-md hover:bg-purple-200 focus:outline-none focus:ring-2 focus:ring-purple-500 disabled:opacity-50 disabled:cursor-not-allowed"
    >
      下一页
    </button>
  </div>
</template>

<script>
export default {
  name: 'PaginationView',
  props: {
    currentPage: {
      type: Number,
      required: true,
    },
    totalPages: {
      type: Number,
      required: true,
    },
  },
  emits: ['update:currentPage'], // 定义事件
  methods: {
    // 上一页
    prevPage() {
      if (this.currentPage > 1) {
        this.$emit('update:currentPage', this.currentPage - 1);
      }
    },
    // 下一页
    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.$emit('update:currentPage', this.currentPage + 1);
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>