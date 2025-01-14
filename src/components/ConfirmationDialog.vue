<template>
  <div
      v-if="isOpen"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center"
  >
    <div class="bg-white p-6 rounded-lg shadow-md w-96">
      <h2 class="text-lg font-semibold mb-4">{{ title }}</h2>
      <p class="text-gray-600 mb-6">{{ message }}</p>
      <div class="flex justify-end space-x-4">
        <button
            @click="onClose"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          取消
        </button>
        <button
            @click="onConfirm"
            class="px-4 py-2 text-sm font-medium text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500"
        >
          确定删除
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ConfirmationDialog',
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
    title: {
      type: String,
      default: '确认删除',
    },
    message: {
      type: String,
      default: '确定删除该项目吗？删除后无法恢复。',
    },
  },
  methods: {
    onClose() {
      this.$emit('close');
    },
    onConfirm() {
      this.$emit('confirm');
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>