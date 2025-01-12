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
            步骤 1: 选择模板
          </div>
          <div
              class="flex-1 text-center"
              :class="{
              'text-purple-800': currentStep === 2,
              'text-gray-500': currentStep !== 2,
            }"
          >
            步骤 2: 选择文件
          </div>
        </div>
        <div class="mt-2 h-1 bg-gray-200 rounded-full">
          <div
              class="h-1 bg-purple-800 rounded-full transition-all"
              :style="{ width: currentStep === 1 ? '50%' : '100%' }"
          ></div>
        </div>
      </div>

      <!-- Step 1: 选择模板类型 -->
      <div v-if="currentStep === 1">
        <label class="block text-sm font-medium text-gray-700 mb-2">选择模板类型</label>
        <select
            v-model="templateType"
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
        >
          <option value="" disabled>请选择模板类型</option>
          <option v-for="option in templateOptions" :key="option" :value="option">
            {{ option }}
          </option>
        </select>
        <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
      </div>

      <!-- Step 2: 选择文件 -->
      <div v-if="currentStep === 2">
        <!-- Query Detail 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">选择 Query Detail 文件</label>
          <div class="mt-1 flex items-center">
            <button
                @click="selectFile('queryDetail')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <p v-if="selectedFiles.queryDetail" class="ml-4 text-sm text-gray-500">
              已选择文件: {{ selectedFiles.queryDetail }}
            </p>
          </div>
        </div>

        <!-- Data Clean Progress 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">选择 Data Clean Progress 文件</label>
          <div class="mt-1 flex items-center">
            <button
                @click="selectFile('dataCleanProgress')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <p v-if="selectedFiles.dataCleanProgress" class="ml-4 text-sm text-gray-500">
              已选择文件: {{ selectedFiles.dataCleanProgress }}
            </p>
          </div>
        </div>

        <!-- Missing Page 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">选择 Missing Page 文件</label>
          <div class="mt-1 flex items-center">
            <button
                @click="selectFile('missingPage')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <p v-if="selectedFiles.missingPage" class="ml-4 text-sm text-gray-500">
              已选择文件: {{ selectedFiles.missingPage }}
            </p>
          </div>
        </div>

        <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
      </div>

      <!-- 操作按钮 -->
      <div class="mt-6 flex justify-between">
        <button
            v-if="currentStep > 1"
            @click="prevStep"
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
            v-if="currentStep < 2"
            @click="nextStep"
            :disabled="currentStep === 1 && !templateType"
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          下一步
        </button>
        <button
            v-if="currentStep === 2"
            @click="confirmImport"
            :disabled="!areFilesSelected"
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          确认导入
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { open } from '@tauri-apps/plugin-dialog'; // 导入 Tauri 的 dialog API

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
      templateType: '',
      templateOptions: [],
      selectedFiles: {
        queryDetail: null, // Query Detail 文件路径
        dataCleanProgress: null, // Data Clean Progress 文件路径
        missingPage: null, // Missing Page 文件路径
      },
      error: '',
    };
  },
  computed: {
    // 检查是否所有文件都已选择
    areFilesSelected() {
      return (
          this.selectedFiles.queryDetail &&
          this.selectedFiles.dataCleanProgress &&
          this.selectedFiles.missingPage
      );
    },
  },
  methods: {
    // 关闭抽屉并清空内容
    close() {
      this.resetForm(); // 清空表单内容
      this.$emit('close'); // 关闭抽屉
    },
    // 获取支持的模板列表
    async fetchSupportedTemplates() {
      try {
        const response = await this.$rustInvoke('fetch_supported_template_list');
        this.templateOptions = response.templates; // 更新模板选项
      } catch (error) {
        console.error('获取支持的模板列表失败:', error);
        this.error = '获取支持的模板列表失败，请重试';
      }
    },
    // 清空表单内容
    resetForm() {
      this.currentStep = 1;
      this.templateType = '';
      this.selectedFiles = {
        queryDetail: null,
        dataCleanProgress: null,
        missingPage: null,
      };
      this.error = '';
    },
    // 选择文件
    async selectFile(type) {
      try {
        const file = await open({
          multiple: false, // 是否允许多选
          filters: [
            {
              name: 'Excel Files',
              extensions: ['xlsx', 'xls', 'csv'], // 限制文件类型
            },
          ],
        });

        if (file) {
          this.selectedFiles[type] = file; // 保存文件路径
          this.error = '';
        }
      } catch (error) {
        console.error('文件选择失败:', error);
        this.error = '文件选择失败，请重试';
      }
    },
    // 下一步
    async nextStep() {
      if (this.currentStep === 1) {
        if (!this.templateType) {
          this.error = '请选择模板类型';
          return;
        }
        this.currentStep = 2;
      }
    },
    // 确认导入
    async confirmImport() {
      try {
        // 显示提醒
        this.$showModal('分析任务正在运行，请稍后查看报告数据');

        const result = await this.$rustInvoke('handle_template_and_files', {
          templateName: this.templateType,
          files: [
            this.selectedFiles.queryDetail,
            this.selectedFiles.dataCleanProgress,
            this.selectedFiles.missingPage,
          ],
        });

        if (result.valid) {
          console.log('文件导入成功:', result);
          this.$emit('confirm-import', [result.data]);
          this.close(); // 关闭抽屉并清空内容
        } else {
          this.error = '文件导入失败，请重试';
        }
      } catch (error) {
        console.error('文件导入失败:', error);
        this.error = '文件导入失败，请重试';
      }
    },
    prevStep() {
      if (this.currentStep > 1) {
        this.currentStep--; // 返回上一步
      }
    },
  },
  // 在组件挂载时获取支持的模板列表
  mounted() {
    this.fetchSupportedTemplates();
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>