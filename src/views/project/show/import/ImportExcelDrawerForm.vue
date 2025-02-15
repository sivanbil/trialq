<template>
  <div
      v-if="isOpen"
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-end"
      @click.self="close"
  >
    <div class="bg-white w-11/12 h-full p-6 overflow-y-auto">
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
            步骤 1: 选择项目
          </div>
          <div
              class="flex-1 text-center"
              :class="{
              'text-purple-800': currentStep === 2,
              'text-gray-500': currentStep !== 2,
            }"
          >
            步骤 2: 选择模板
          </div>
          <div
              class="flex-1 text-center"
              :class="{
              'text-purple-800': currentStep === 3,
              'text-gray-500': currentStep !== 3,
            }"
          >
            步骤 3: 选择数据文件
          </div>
          <div
              class="flex-1 text-center"
              :class="{
              'text-purple-800': currentStep === 4,
              'text-gray-500': currentStep !== 4,
            }"
          >
            步骤 4: 数据分析
          </div>
        </div>
        <div class="mt-2 h-1 bg-gray-200 rounded-full">
          <div
              class="h-1 bg-purple-800 rounded-full transition-all"
              :style="{
              width:
                currentStep === 1
                  ? '25%'
                  : currentStep === 2
                  ? '50%'
                  : currentStep === 3
                  ? '75%'
                  : '100%',
            }"
          ></div>
        </div>
      </div>

      <!-- Step 1: 选择项目 -->
      <div v-if="currentStep === 1">
        <label class="block text-sm font-medium text-gray-700 mb-2"
        >选择项目</label
        >
        <select
            v-model="project"
            @change="fetchSupportedTemplates"
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
        >
          <option value="" disabled>请选择项目</option>
          <option v-for="option in projectOptions" :key="option.id" :value="option.project_name">
            {{ option.project_name }}
          </option>
        </select>
        <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
      </div>

      <!-- Step 2: 选择模板类型 -->
      <div v-if="currentStep === 2">
        <label class="block text-sm font-medium text-gray-700 mb-2"
        >选择模板类型</label
        >
        <select
            v-model="templateType"
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500 sm:text-sm"
        >
          <option value="" disabled>请选择模板类型</option>
          <option
              v-for="option in templateOptions"
              :key="option"
              :value="option"
          >
            {{ option }}
          </option>
        </select>
        <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
      </div>

      <!-- Step 3: 选择文件 -->
      <div v-if="currentStep === 3">
        <!-- Query Detail 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2"
          >选择 Query Detail 文件</label
          >
          <div class="mt-1 flex items-center">
            <button
                @click="selectFile('queryDetail')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <p
                v-if="selectedFiles.queryDetail"
                class="ml-4 text-sm text-gray-500"
            >
              已选择文件: {{ selectedFiles.queryDetail }}
            </p>
          </div>
        </div>

        <!-- Data Clean Progress 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2"
          >选择 Data Clean Progress 文件</label
          >
          <div class="mt-1 flex items-center">
            <button
                @click="selectFile('dataCleanProgress')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <p
                v-if="selectedFiles.dataCleanProgress"
                class="ml-4 text-sm text-gray-500"
            >
              已选择文件: {{ selectedFiles.dataCleanProgress }}
            </p>
          </div>
        </div>

        <!-- Missing Page 文件选择 -->
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2"
          >选择 Missing Page 文件</label
          >
          <div class="mt-1 flex items-center">
            <button
                @click="selectFile('missingPage')"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
            >
              选择文件
            </button>
            <p
                v-if="selectedFiles.missingPage"
                class="ml-4 text-sm text-gray-500"
            >
              已选择文件: {{ selectedFiles.missingPage }}
            </p>
          </div>
        </div>

        <p v-if="error" class="text-sm text-red-500 mt-2">{{ error }}</p>
      </div>

      <!-- Step 4: 数据分析 -->
      <div v-if="currentStep === 4">
        <div class="mb-4">
          <label class="block text-sm font-medium text-gray-700 mb-2"
          >接下来可以进行数据分析</label
          >

        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="mt-6 flex justify-between">
        <button
            v-if="currentStep > 1 && currentStep <= 3"
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
          关闭
        </button>
        <button
            v-if="currentStep < 4"
            @click="nextStep"
            :disabled="
            (currentStep === 1 && !project) ||
            (currentStep === 2 && !templateType) ||
            (currentStep === 3 && !areFilesSelected)
          "
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500 disabled:bg-gray-300 disabled:cursor-not-allowed"
        >
          <span v-if="currentStep === 3">导入</span>
          <span v-else>下一步</span>
        </button>
        <button
            v-if="currentStep === 4"
            @click="analyzeData"
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
        >
          开始分析
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { open } from "@tauri-apps/plugin-dialog"; // 导入 Tauri 的 dialog API

export default {
  name: "DrawerForm",
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
  },
  data() {
    return {
      currentStep: 1,
      project: "", // 当前选择的项目
      projectOptions: [], // 项目列表
      templateType: "", // 当前选择的模板类型
      templateOptions: [], // 模板列表
      selectedFiles: {
        queryDetail: null, // Query Detail 文件路径
        dataCleanProgress: null, // Data Clean Progress 文件路径
        missingPage: null, // Missing Page 文件路径
      },
      analysisResult: "", // 数据分析结果
      reportNo: "",
      error: "",
    };
  },
  computed: {
    // 检查是否所有文件都已选择
    areFilesSelected() {
      return (
          this.selectedFiles.queryDetail &&
          this.selectedFiles.dataCleanProgress
      );
    },
  },
  methods: {
    // 关闭抽屉并清空内容
    close() {
      this.resetForm(); // 清空表单内容
      this.$emit("close"); // 关闭抽屉
    },
    // 获取项目列表
    async fetchProjects() {
      try {
        const response = await this.$rustInvoke("fetch_project_list", {
          currentPage: 1,
          pageSize: 100,
          keyword: "",
        });
        this.projectOptions = response.projects; // 更新项目选项
      } catch (error) {
        console.error("获取项目列表失败:", error);
        this.error = "获取项目列表失败，请重试";
      }
    },
    // 获取支持的模板列表
    async fetchSupportedTemplates() {
      try {
        const response = await this.$rustInvoke("fetch_supported_template_list");
        this.templateOptions = response.templates; // 更新模板选项
      } catch (error) {
        console.error("获取支持的模板列表失败:", error);
        this.error = "获取支持的模板列表失败，请重试";
      }
    },
    // 清空表单内容
    resetForm() {
      this.currentStep = 1;
      this.project = "";
      this.templateType = "";
      this.selectedFiles = {
        queryDetail: null,
        dataCleanProgress: null,
        missingPage: null,
      };
      this.analysisResult = "";
      this.error = "";
    },
    // 选择文件
    async selectFile(type) {
      try {
        const file = await open({
          multiple: false, // 是否允许多选
          filters: [
            {
              name: "Excel Files",
              extensions: ["xlsx", "xls", "csv"], // 限制文件类型
            },
          ],
        });

        if (file) {
          this.selectedFiles[type] = file; // 保存文件路径
          this.error = ""; // 清空错误信息
        }
      } catch (error) {
        console.error("文件选择失败:", error);
        this.error = "文件选择失败，请重试";
      }
    },
    // 下一步
    async nextStep() {
      if (this.currentStep === 1) {
        if (!this.project) {
          this.error = "请选择项目";
          return;
        }
        try {
          const response = await this.$rustInvoke('fetch_site_list', {
            projectNo: this.project,
            currentPage: 1,
            pageSize: 10,
          });
          if (!response.valid) {
            return this.$showModal("请去项目列表维护中心数据!");
          }
        } catch (error) {
          return this.$showModal("网络异常");
        }
        this.currentStep = 2;
      } else if (this.currentStep === 2) {
        if (!this.templateType) {
          this.error = "请选择模板类型";
          return;
        }
        this.currentStep = 3;
      } else if (this.currentStep === 3) {
        if (!this.areFilesSelected) {
          this.error = "请选择所有文件";
          return;
        }
        await this.confirmImport(); // 调用确认导入方法
      }
      this.error = ""; // 清空错误信息
    },
    // 上一步
    prevStep() {
      if (this.currentStep > 1) {
        this.currentStep--; // 返回上一步
        this.error = ""; // 清空错误信息
      }
    },
    // 确认导入
    async confirmImport() {
      // 显示不可关闭的提示框
      const closeModal = this.$showModal("系统正在导入数据，请不要关闭程序...", {
        showCloseButton: false, // 隐藏关闭按钮
        allowOutsideClick: false, // 禁止点击外部关闭
        useListener: true,
        eventName: "import_data_progress"
      });
      try {


        // 调用导入接口
        const result = await this.$rustInvoke("handle_template_and_files", {
          projectNo: this.project, // 传递当前选择的项目
          templateName: this.templateType,
          files: [
            this.selectedFiles.queryDetail,
            this.selectedFiles.dataCleanProgress,
            this.selectedFiles.missingPage == null ? '' : this.selectedFiles.missingPage,
          ],
        });
        if (typeof result === 'object' && result!== null) {
          if (result.valid) {
            this.reportNo = result.data.reportNumber
            console.log("文件导入成功:", result);
            closeModal(); // 关闭提示框
            this.currentStep = 4; // 切换到第四步

            await this.analyzeData();
          } else {
            closeModal(); // 关闭提示框
            this.error = "文件导入失败，请重试";
          }
        } else {
          closeModal(); // 关闭提示框
          this.$showModal(result);
        }
      } catch (error) {
        closeModal(); // 关闭提示框
        this.$showModal("部分文件导入失败,如果字符集问题，尽量是通过拷贝所有数据到新的excel表格里:（"+error+"),如果生成了记录，可以将记录删除");
        this.error = "文件导入失败，请重试";
      }
    },
    // 数据分析
    async analyzeData() {
      // 显示不可关闭的提示框
      const closeModal = this.$showModal("系统正在分析数据，请稍候...", {
        showCloseButton: false, // 隐藏关闭按钮
        allowOutsideClick: false, // 禁止点击外部关闭
      });

      try {
        // 调用数据分析接口
        const result = await this.$rustInvoke("analyze_report_data", {
          projectNumber: this.project,
          reportNumber: this.reportNo
        });

        if (result.valid) {
          console.log("数据分析成功:", result);
          this.analysisResult = result.data; // 更新分析结果
        } else {
          this.error = "数据分析失败，请重试";
        }
        this.close(); // 关闭抽屉
      } catch (error) {
        console.error("数据分析失败:", error);
        this.error = "数据分析失败，请重试";
      }
      // 关闭提示框和抽屉
      closeModal();
    },
  },
  // 在组件挂载时获取项目列表
  mounted() {
    this.fetchProjects();
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>
