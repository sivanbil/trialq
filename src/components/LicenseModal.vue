<template>
  <div v-if="visible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 text-left">
    <div class="bg-white p-6 rounded-lg shadow-lg w-2/3" style="margin-top:120px;">
      <h2 class="text-xl font-semibold mb-4">输入注册码</h2>
      <form @submit.prevent="handleSubmit">
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700">邮箱</label>
          <input
              v-model="form.email"
              type="email"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
              required
          />
        </div>
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700">注册码</label>
          <textarea
              v-model="form.licenseKey"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
              rows="6"
          required
          />
        </div>
        <div class="flex justify-end space-x-2">
          <button
              type="button"
              @click="close"
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500"
          >
            取消
          </button>
          <button
              type="submit"
              class="px-4 py-2 text-sm font-medium text-white bg-purple-800 rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
          >
            保存
          </button>
        </div>
      </form>
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
  },
  data() {
    return {
      form: {
        email: '',
        licenseKey: '',
      },
    };
  },
  methods: {
    close() {
      this.$emit('close');
    },
    handleSubmit() {
      this.$emit('save', this.form);
      this.close(); // 保存后关闭弹窗
    },
  },
};
</script>