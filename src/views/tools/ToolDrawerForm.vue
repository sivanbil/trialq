<template>
  <div
      v-if="isOpen"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-end text-left"
      @click.self="close"
  >
    <div class="bg-white w-2/3 h-full p-6 overflow-y-auto">
      <h2 class="text-lg font-semibold mb-4">添加工具</h2>
      <form @submit.prevent="save" class="space-y-4">
        <!-- 工具名称 -->
        <div>
          <label class="block text-sm font-medium text-gray-700">工具名称</label>
          <input
              v-model="form.name"
              type="text"
              placeholder="工具名称"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
              :class="{ 'border-red-500': errors.name }"
          />
          <p v-if="errors.name" class="text-sm text-red-500 mt-1">{{ errors.name }}</p>
        </div>

        <!-- 工具链接地址 -->
        <div>
          <label class="block text-sm font-medium text-gray-700">工具链接地址</label>
          <input
              v-model="form.url"
              type="text"
              placeholder="工具链接地址"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
              :class="{ 'border-red-500': errors.url }"
          />
          <p v-if="errors.url" class="text-sm text-red-500 mt-1">{{ errors.url }}</p>
        </div>

        <!-- 操作按钮 -->
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
  name: 'ToolFormDrawer',
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
  },
  data() {
    return {
      form: {
        name: '',
        url: '',
      },
      errors: {
        name: '',
        url: '',
      },
    };
  },
  methods: {
    // 校验表单
    validateForm() {
      this.errors = { name: '', url: '' }; // 清空错误信息

      if (!this.form.name) {
        this.errors.name = '工具名称不能为空';
      }
      if (!this.form.url) {
        this.errors.url = '工具链接地址不能为空';
      }

      // 如果没有任何错误，返回 true
      return !this.errors.name && !this.errors.url;
    },
    // 关闭抽屉
    close() {
      this.$emit('close');
    },
    // 保存工具
    save() {
      if (this.validateForm()) {
        this.$emit('save', this.form);
        this.form = { name: '', url: '' }; // 清空表单
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>