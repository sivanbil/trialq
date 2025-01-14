<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
    <div class="bg-white p-6 rounded-lg shadow-md w-11/12 max-h-[90vh] overflow-y-auto relative">
      <!-- 关闭按钮 -->
      <button
          @click="closeDialog"
          class="absolute top-4 right-4 p-2 text-gray-500 hover:text-gray-700 focus:outline-none"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>

      <h2 class="text-lg font-semibold mb-4">项目中心</h2>

      <!-- 添加中心按钮 -->
      <div class="mb-4 text-right">
        <button
            @click="openCenterForm"
            class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          添加中心
        </button>
      </div>

      <!-- 中心列表 -->
      <div v-if="!isFormOpen">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              中心编号
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              中心名称
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              负责 CRA
            </th>
            <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              操作
            </th>
          </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="center in centers" :key="center.site_number">
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ center.site_number }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ center.site_name }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ center.site_cra }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              <button
                  @click="openCenterForm(center)"
                  class="px-2 py-1 text-sm text-blue-500 hover:text-blue-700"
              >
                编辑
              </button>
              <button
                  @click="deleteCenter(center.id)"
                  class="px-2 py-1 text-sm text-red-500 hover:text-red-700"
              >
                删除
              </button>
            </td>
          </tr>
          </tbody>
        </table>

        <!-- 分页控件 -->
        <div class="mt-1">
          <!-- 分页控件 -->
          <Pagination
              v-if="centers.length > 0"
              :currentPage="currentPage"
              :totalPages="totalPages"
              @update:currentPage="handlePageChange"
          />
        </div>

        </div>

      <!-- 添加/编辑中心表单 -->
      <div v-if="isFormOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
        <div class="bg-white p-6 rounded-lg shadow-md w-11/12">
          <h3 class="text-lg font-semibold mb-4">{{ formData.siteId ? '编辑中心' : '添加中心' }}</h3>
          <form @submit.prevent="handleSubmit">
            <div class="space-y-4">
              <!-- 中心编号 -->
              <div>
                <label class="block text-sm font-medium text-gray-700">
                  中心编号
                  <span class="text-red-500">*</span>
                </label>
                <input
                    v-model="formData.siteNumber"
                    type="text"
                    placeholder="请输入中心编号"
                    required
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500"
                />
              </div>
              <!-- 中心名称 -->
              <div>
                <label class="block text-sm font-medium text-gray-700">
                  中心名称
                </label>
                <input
                    v-model="formData.name"
                    type="text"
                    placeholder="请输入中心名称"
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500"
                />
              </div>
              <!-- 负责 CRA -->
              <div>
                <label class="block text-sm font-medium text-gray-700">
                  负责 CRA
                </label>
                <input
                    v-model="formData.manager"
                    type="text"
                    placeholder="请输入负责 CRA"
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-purple-500 focus:border-purple-500"
                />
              </div>
            </div>
            <div class="mt-6 flex justify-end space-x-4">
              <button
                  type="button"
                  @click="closeForm"
                  class="px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300"
              >
                取消
              </button>
              <button
                  type="submit"
                  class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700"
              >
                保存
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- 批量导入抽屉 -->
      <div v-if="isImportDrawerOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-end">
        <div class="bg-white p-6 rounded-lg shadow-md w-11/12 h-full">
          <h3 class="text-lg font-semibold mb-4">批量导入中心</h3>
          <div>
            <label class="block text-sm font-medium text-gray-700">
              选择文件
              <span class="text-red-500">*</span>
            </label>
            <button
                @click="openFilePicker"
                class="mt-1 block w-full px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300 text-left"
            >
              {{ filePath ? filePath : '点击选择文件' }}
            </button>
          </div>
          <div class="mt-6 flex justify-end space-x-4">
            <button
                type="button"
                @click="isImportDrawerOpen = false"
                class="px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300"
            >
              取消
            </button>
            <button
                @click="handleFileUpload"
                :disabled="!filePath"
                class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 disabled:opacity-50"
            >
              导入
            </button>
          </div>
        </div>
      </div>

      <!-- 批量导入按钮 -->
      <div class="mt-6 text-right">
        <button
            @click="isImportDrawerOpen = true"
            class="px-4 py-2 bg-purple-700 text-white rounded-md hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500"
        >
          批量导入
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { open } from "@tauri-apps/plugin-dialog";
import Pagination from "@/components/PaginationView.vue"; // 导入 Tauri 的 dialog API

export default {
  name: 'SiteManage',
  components: {Pagination},
  props: {
    projectNumber: {
      type: String,
      required: true,
    },
  },
  data() {
    return {
      centers: [], // 中心列表数据
      isFormOpen: false, // 是否显示表单
      isImportDrawerOpen: false, // 是否显示批量导入抽屉
      currentPage: 1, // 当前页码
      pageSize: 10, // 每页显示条数
      totalPages: 1, // 总页数
      formData: {
        siteId: null, // 编辑时使用的中心 ID
        siteNumber: '',
        name: '',
        manager: '',
      }, // 表单数据
      filePath: '', // 选择的文件路径
    };
  },
  methods: {
    // 关闭弹窗
    closeDialog() {
      this.$emit('close');
    },
    // 处理页码变化
    handlePageChange(newPage) {
      this.currentPage = newPage;
      this.fetchCenterData();
    },
    // 获取中心列表数据
    async fetchCenterData() {
      try {
        const response = await this.$rustInvoke('fetch_site_list', {
          projectNo: this.projectNumber,
          currentPage: this.currentPage,
          pageSize: this.pageSize,
        });
        if (response.valid) {
          this.centers = response.sites;
          this.totalPages = Math.ceil(response.total / this.pageSize);
        } else {
          console.error('获取中心列表失败:', response.message);
        }
      } catch (error) {
        console.error('调用获取中心列表接口失败:', error);
      }
    },

    // 打开表单（添加或编辑）
    openCenterForm(center = null) {
      this.isFormOpen = true;
      if (center) {
        // 编辑模式
        this.formData = {
          siteId: center.id,
          siteNumber: center.site_number,
          name: center.site_name,
          manager: center.site_cra,
        };
      } else {
        // 添加模式
        this.formData = {
          siteId: null,
          siteNumber: '',
          name: '',
          manager: '',
        };
      }
    },

    // 关闭表单
    closeForm() {
      this.isFormOpen = false;
    },

    // 处理表单提交（添加或编辑）
    async handleSubmit() {
      if (!this.formData.siteNumber) {
        this.$showModal('中心编号为必填项！');
        return;
      }

      try {
        const payload = {
          siteNumber: this.formData.siteNumber,
          siteName: this.formData.name,
          siteCra: this.formData.manager,
          projectName: this.projectNumber,
        };

        let response;
        if (this.formData.siteId) {
          // 编辑模式
          payload.siteId = this.formData.siteId;
          response = await this.$rustInvoke('update_site_by_id', payload);
        } else {
          // 添加模式
          response = await this.$rustInvoke('save_project_site', payload);
        }

        if (response.valid) {
          this.$showModal(this.formData.siteId ? '中心更新成功！' : '中心添加成功！');
          await this.fetchCenterData(); // 重新加载数据
          this.closeForm(); // 关闭表单
        } else {
          this.$showModal(this.formData.siteId ? '中心更新失败' : '中心添加失败');
        }
      } catch (error) {
        this.$showModal(this.formData.siteId ? '中心更新失败，请重试！' : '中心添加失败，请重试！');
      }
    },

    // 删除中心
    async deleteCenter(siteId) {
      try {
        const response = await this.$rustInvoke('delete_project_site', {
          siteId: siteId,
        });
        if (response.valid) {
          this.$showModal('删除成功！');
          await this.fetchCenterData(); // 重新加载数据
        } else {
          this.$showModal('删除失败');
        }
      } catch (error) {
        this.$showModal('删除失败，请重试！');
      }
    },

    // 打开文件选择器
    async openFilePicker() {
      try {
        const selected = await open({
          multiple: false,
          filters: [{ name: 'Excel Files', extensions: ['xlsx', 'xls'] }],
        });
        if (selected) {
          this.filePath = selected; // 保存文件路径
        }
      } catch (error) {
        console.error('文件选择失败:', error);
        this.$showModal('文件选择失败，请重试！');
      }
    },

    // 处理文件上传（批量导入）
    async handleFileUpload() {
      if (!this.filePath) {
        this.$showModal('请先选择文件！');
        return;
      }

      try {
        const response = await this.$rustInvoke('handle_site_file', {
          filePath: this.filePath,
          projectNumber: this.projectNumber
        });
        if (response.valid) {
          this.$showModal('导入成功！');
          await this.fetchCenterData(); // 重新加载数据
          this.isImportDrawerOpen = false; // 关闭导入抽屉
          this.filePath = ''; // 清空文件路径
        } else {
          this.$showModal('导入失败：' + response.message);
        }
      } catch (error) {
        console.error('调用导入接口失败:', error);
        this.$showModal('导入失败，请重试！');
      }
    },

    // 上一页
    prevPage() {
      if (this.currentPage > 1) {
        this.currentPage--;
        this.fetchCenterData();
      }
    },

    // 下一页
    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.currentPage++;
        this.fetchCenterData();
      }
    },
  },
  mounted() {
    this.fetchCenterData();
  },
};
</script>

<style scoped>
/* 添加自定义样式 */
</style>