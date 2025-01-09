<template>
  <div
      v-if="isOpen"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-end"
      @click.self="close"
  >
    <div class="bg-white w-2/3 h-full p-6 overflow-y-auto">
      <h3 class="text-xl font-semibold mb-4">导入表格</h3>

      <!-- Step 步骤条 -->
      <div class="mb-6">
        <div class="flex justify-between">
          <div
              class="flex-1 text-center"
              :class="{
              'text-purple-800': currentStep === 1,
              'text-gray-500': currentStep !== 1,
            }"
          >
            步骤 1: 选择文件
          </div>
          <div
              class="flex-1 text-center"
              :class="{
              'text-purple-800': currentStep === 2,
              'text-gray-500': currentStep !== 2,
            }"
          >
            步骤 2: 数据预览
          </div>
        </div>
        <div class="mt-2 h-1 bg-gray-200 rounded-full">
          <div
              class="h-1 bg-purple-800 rounded-full transition-all"
              :style="{ width: currentStep === 1 ? '50%' : '100%' }"
          ></div>
        </div>
      </div>

      <!-- Step 1: 选择文件 -->
      <div v-if="currentStep === 1">
        <label class="block text-sm font-medium text-gray-700 mb-2">选择 Excel 文件</label>
        <input
            type="file"
            accept=".xlsx, .xls"
            @change="handleFileUpload"
            class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-purple-800 file:text-white hover:file:bg-purple-700"
        />
        <p v-if="uploading" class="text-sm text-gray-500 mt-2">文件上传中，请稍候...</p>
        <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
      </div>

      <!-- Step 2: 数据预览 -->
      <div v-if="currentStep === 2">
        <h4 class="text-lg font-semibold mb-2">数据预览</h4>
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">日期</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">项目编号</th>
          </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="(item, index) in previewData" :key="index">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.date }}</td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{{ item.projectId }}</td>
          </tr>
          </tbody>
        </table>
      </div>

      <!-- 操作按钮 -->
      <div class="mt-6 flex justify-between">
        <button
            v-if="currentStep === 2"
            @click="currentStep = 1"
            class="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          上一步
        </button>
        <button
            v-else
            @click="close"
            class="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          取消
        </button>
        <button
            v-if="currentStep === 1"
            @click="nextStep"
            :disabled="!file"
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          下一步
        </button>
        <button
            v-if="currentStep === 2"
            @click="confirmImport"
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
        >
          确认导入
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'DrawerForm',
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
  },
  data() {
    return {
      currentStep: 1,
      file: null,
      uploading: false,
      error: '',
      previewData: [], // 预览数据
    };
  },
  methods: {
    // 关闭抽屉
    close() {
      this.$emit('close');
    },
    // 处理文件上传
    handleFileUpload(event) {
      this.file = event.target.files[0];
    },
    // 下一步
    async nextStep() {
      if (!this.file) {
        this.error = '请选择文件';
        return;
      }

      this.uploading = true;
      this.error = '';

      try {
        // 模拟上传文件到后端
        const formData = new FormData();
        formData.append('file', this.file);

        // 这里替换为实际的上传逻辑
        const response = await fetch('/api/upload-excel', {
          method: 'POST',
          body: formData,
        });

        if (!response.ok) {
          throw new Error('上传失败');
        }

        // 假设后端返回解析后的数据
        const data = await response.json();
        this.previewData = data;

        // 进入下一步
        this.currentStep = 2;
      } catch (err) {
        this.error = '文件上传失败，请重试';
        console.error('文件上传失败:', err);
      } finally {
        this.uploading = false;
      }
    },
    // 确认导入
    confirmImport() {
      this.$emit('confirm-import', this.previewData);
      this.close();
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>