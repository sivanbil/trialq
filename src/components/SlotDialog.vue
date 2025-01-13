<template>
  <!-- 遮罩层 -->
  <div
      v-if="isOpen"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center"
      @click.self="closeDialog"
  >
    <!-- 对话框内容 -->
    <div class="bg-white rounded-lg shadow-lg w-11/12 p-6">
      <!-- 标题 -->
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-xl font-semibold">{{ title }}</h3>
        <button @click="closeDialog" class="text-gray-500 hover:text-gray-700">
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

      <!-- 内容插槽 -->
      <div class="mb-4">
        <slot></slot>
      </div>

      <!-- 操作按钮 -->
      <div class="flex justify-end space-x-4">
        <button
            @click="closeDialog"
            v-if="showCancel"
            class="px-4 py-2 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition duration-200"
        >
          取消
        </button>
        <button
            v-if="showConfirm"
            @click="confirmDialog"
            class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition duration-200"
        >
          确认
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'SlotDialog',
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
    title: {
      type: String,
      default: '操作提示',
    },
    showConfirm: {
      type: Boolean,
      required: true,
    },
    showCancel: {
      type: Boolean,
      required: false,
      default: false,
    }
  },
  watch: {
    isOpen(newVal) {
      if (newVal) {
        // 对话框打开时的逻辑
        console.log('Dialog opened');
      } else {
        // 对话框关闭时的逻辑
        console.log('Dialog closed');
      }
    },
  },
  methods: {
    // 关闭对话框
    closeDialog() {
      this.$emit('close');
    },
    // 确认对话框
    confirmDialog() {
      this.$emit('confirm');
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>