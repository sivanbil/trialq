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
            <input
                type="file"
                ref="queryDetailInput"
                @change="(event) => handleFileSelect(event, 'queryDetail')"
                class="hidden"
            />
            <button
                @click="triggerFileInput('queryDetail')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <button
                v-if="selectedFiles.queryDetail"
                @click="previewFile('queryDetail')"
                class="ml-2 p-2 text-purple-800 hover:text-purple-700 focus:outline-none"
            >
              👁️
            </button>
            <p v-if="selectedFiles.queryDetail" class="ml-4 text-sm text-gray-500">
              已选择文件: {{ selectedFiles.queryDetail.name }}
            </p>
          </div>
        </div>

        <!-- Data Clean Progress 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">选择 Data Clean Progress 文件</label>
          <div class="mt-1 flex items-center">
            <input
                type="file"
                ref="dataCleanProgressInput"
                @change="(event) => handleFileSelect(event, 'dataCleanProgress')"
                class="hidden"
            />
            <button
                @click="triggerFileInput('dataCleanProgress')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <button
                v-if="selectedFiles.dataCleanProgress"
                @click="previewFile('dataCleanProgress')"
                class="ml-2 p-2 text-purple-800 hover:text-purple-700 focus:outline-none"
            >
              👁️
            </button>
            <p v-if="selectedFiles.dataCleanProgress" class="ml-4 text-sm text-gray-500">
              已选择文件: {{ selectedFiles.dataCleanProgress.name }}
            </p>
          </div>
        </div>

        <!-- Missing Page 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2">选择 Missing Page 文件</label>
          <div class="mt-1 flex items-center">
            <input
                type="file"
                ref="missingPageInput"
                @change="(event) => handleFileSelect(event, 'missingPage')"
                class="hidden"
            />
            <button
                @click="triggerFileInput('missingPage')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <button
                v-if="selectedFiles.missingPage"
                @click="previewFile('missingPage')"
                class="ml-2 p-2 text-purple-800 hover:text-purple-700 focus:outline-none"
            >
              👁️
            </button>
            <p v-if="selectedFiles.missingPage" class="ml-4 text-sm text-gray-500">
              已选择文件: {{ selectedFiles.missingPage.name }}
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

      <!-- 文件预览模态框 -->
      <FilePreviewModal
          :is-visible="isPreviewVisible"
          :file-data="previewFileData"
          @close="closePreview"
      />
    </div>
  </div>
</template>

<script>
import * as XLSX from 'xlsx'; // 导入 xlsx 库
import FilePreviewModal from '@/components/FilePreviewModal.vue'; // 导入文件预览组件

export default {
  name: 'DrawerForm',
  components: {
    FilePreviewModal,
  },
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
      templateOptions: ['medidata'],
      selectedFiles: {
        queryDetail: null, // Query Detail 文件
        dataCleanProgress: null, // Data Clean Progress 文件
        missingPage: null, // Missing Page 文件
      },
      error: '',
      isPreviewVisible: false, // 控制文件预览模态框的显示
      previewFileData: null, // 文件预览数据
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
    // 关闭抽屉
    close() {
      this.$emit('close');
    },
    // 触发文件选择
    triggerFileInput(type) {
      this.$refs[`${type}Input`].click();
    },
    // 处理文件选择
    handleFileSelect(event, type) {
      const file = event.target.files[0];
      if (file) {
        this.selectedFiles[type] = file;
        this.error = '';
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
    // 预览文件
    async previewFile(type) {
      const file = this.selectedFiles[type];
      if (file) {
        if (file.size > 5 * 1024 * 1024) {
          this.$showModal('文件过大，无法在线预览'); // 提示用户文件过大
          return;
        }
        const data = await this.parseFile(file);
        this.previewFileData = data;
        this.isPreviewVisible = true; // 显示文件预览模态框
      }
    },
    // 解析文件
    async parseFile(file) {
      return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = (e) => {
          const data = new Uint8Array(e.target.result);
          const workbook = XLSX.read(data, {type: 'array'});
          const sheetName = workbook.SheetNames[0];
          const sheet = workbook.Sheets[sheetName];
          const json = XLSX.utils.sheet_to_json(sheet, {header: 1});

          const headers = json[0]; // 第一行为表头
          const rows = json.slice(1); // 其余为数据行

          resolve({headers, rows});
        };
        reader.onerror = (error) => reject(error);
        reader.readAsArrayBuffer(file);
      });
    },
    // 关闭文件预览
    closePreview() {
      this.isPreviewVisible = false;
    },
    // 确认导入
    confirmImport() {
      this.$emit('confirm-import', this.selectedFiles);
      this.close();
    },
    prevStep() {
      if (this.currentStep > 1) {
        this.currentStep--; // 返回上一步
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>