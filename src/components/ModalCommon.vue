<template>
  <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" style="z-index: 200">
    <div class="bg-white p-6 rounded-lg shadow-lg w-96">
      <h2 class="text-xl font-semibold mb-4">{{ title }}</h2>
      <p class="text-gray-700 mb-4 modal-message">{{ message }}</p>
      <div class="flex justify-end" v-if="showCloseButton">
        <button
            @click="close"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    visible: {
      type: Boolean,
      required: true,
    },
    title: {
      type: String,
      default: '提示',
    },
    message: {
      type: String,
      default: '',
    },
    showCloseButton: {
      type: Boolean,
      default: true, // 默认显示关闭按钮
    },
    onClose: {
      type: Function,
      default: () => {
      },
    },
  },
  methods: {
    close() {
      this.onClose(); // 调用关闭回调
    },
  },
};
</script>
