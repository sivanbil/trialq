<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-end" @click.self="$emit('close')">
    <div class="bg-white w-11/12 h-full p-6 overflow-y-auto">
      <h2 class="text-lg font-semibold mb-4">批量导入中心数据</h2>
      <form @submit.prevent="$emit('submit', selectedFile)">
        <div class="space-y-4">
          <!-- 文件选择 -->
          <div>
            <label class="block text-sm font-medium text-gray-700">
              选择 Excel 文件
              <span class="text-red-500">*</span>
            </label>
            <input
                type="file"
                accept=".xlsx, .xls"
                @change="handleFileChange"
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500"
            />
            <p v-if="error" class="text-sm text-red-500 mt-1">{{ error }}</p>
          </div>
        </div>
        <div class="mt-6 flex justify-end space-x-4">
          <button
              type="button"
              @click="$emit('close')"
              class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-gray-500"
          >
            取消
          </button>
          <button
              type="submit"
              class="px-4 py-2 text-sm font-medium text-white bg-purple-800 rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
          >
            导入
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ImportSiteDrawer',
  data() {
    return {
      selectedFile: null,
      error: '',
    };
  },
  methods: {
    handleFileChange(event) {
      const file = event.target.files[0];
      if (file) {
        if (file.name.endsWith('.xlsx') || file.name.endsWith('.xls')) {
          this.selectedFile = file;
          this.error = '';
        } else {
          this.error = '请选择有效的 Excel 文件（.xlsx 或 .xls）';
        }
      }
    },
  },
};
</script>