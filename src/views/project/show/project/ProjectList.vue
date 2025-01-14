<template>
  <div class="bg-white p-6 rounded-lg shadow-md text-left">
    <!-- 添加项目按钮和搜索框 -->
    <div class="mb-6 flex justify-between items-center">
      <div class="flex space-x-4">
        <input
            v-model="searchKeyword"
            type="text"
            placeholder="输入项目名称检索"
            class="px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
        />
        <button
            @click="fetchProjects"
            class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
        >
          搜索
        </button>
      </div>
      <button
          @click="openDrawer"
          class="px-4 py-2 bg-purple-800 text-white rounded-md hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-purple-500"
      >
        添加项目
      </button>
    </div>

    <!-- 项目列表 -->
    <div>
      <h2 class="text-lg font-semibold mb-4">项目列表</h2>
      <div class="text-center py-6 text-gray-500" v-if="!projects || projects.length === 0">
        没有任何项目数据，请添加下
      </div>
      <ul v-else class="space-y-4">
        <li
            v-for="(project, index) in projects"
            :key="index"
            class="p-4 border border-gray-200 rounded-lg hover:shadow-md transition-shadow"
        >
          <div class="flex justify-between items-center">
            <div>
              <h3 class="text-lg font-semibold text-purple-800">{{ project.project_name }}</h3>
              <p class="text-sm text-gray-600">{{ project.description }}</p>
            </div>
            <div class="flex space-x-2">
              <button
                  class="px-3 py-1 text-sm font-medium text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  @click="viewRelatedReportList(project.project_name)"
              >
                查看相关数据报告
              </button>

              <button
                  class="px-3 py-1 text-sm font-medium text-white bg-green-500 rounded-md hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500"
                  @click="openSiteMaintenance(project.project_name)"
              >
                维护中心数据
              </button>

              <button
                  @click="openDeleteConfirmation(index)"
                  class="px-3 py-1 text-sm font-medium text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-500"
              >
                删除
              </button>
            </div>
          </div>
        </li>
      </ul>
    </div>

    <!-- 分页控件 -->
    <div v-if="projects.length > 0" class="mt-6 flex justify-center space-x-4">
      <button
          @click="prevPage"
          :disabled="currentPage === 1"
          class="px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        上一页
      </button>
      <span class="text-sm text-gray-600">第 {{ currentPage }} 页 / 共 {{ totalPages }} 页</span>
      <button
          @click="nextPage"
          :disabled="currentPage === totalPages"
          class="px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        下一页
      </button>
    </div>

    <!-- 抽屉表单 -->
    <ProjectFormDrawer
        :isOpen="isDrawerOpen"
        :editedProject="editedProject"
        @close="closeDrawer"
        @save="handleSave"
    />

    <!-- 自定义确认弹窗 -->
    <ConfirmationDialog
        :isOpen="isDeleteConfirmationOpen"
        @close="closeDeleteConfirmation"
        @confirm="confirmDelete"
    />

    <SiteManagement :projectNumber="projectNumber" @close="closeSiteDialog" v-if="isSiteDialogOpen"/>
  </div>

  <SlotDialog :showConfirm="false" :isOpen="isDialogOpen" title="报告归档" @close="closeDialog">
    <ReportList :projectNumber="projectNumber" v-if="projectNumber"/>
  </SlotDialog>
</template>

<script>
import ProjectFormDrawer from './ProjectDrawerForm.vue';
import SlotDialog from "@/components/SlotDialog.vue";
import ReportList from "@/views/project/show/project/ReportList.vue";
import SiteManagement from "@/views/project/show/import/SiteManage.vue";
import ConfirmationDialog from "@/components/ConfirmationDialog.vue"; // 引入确认弹窗组件

export default {
  name: 'ProjectList',
  components: {
    SiteManagement,
    ReportList,
    SlotDialog,
    ProjectFormDrawer,
    ConfirmationDialog,
  },
  data() {
    return {
      isDialogOpen: false,
      projects: [],
      currentPage: 1,
      pageSize: 10,
      totalProjects: 0,
      searchKeyword: '',
      projectNumber: "",
      isDrawerOpen: false,
      editedProject: {
        index: null,
        name: '',
        description: '',
      },
      isSiteDialogOpen: false,
      isDeleteConfirmationOpen: false, // 控制确认弹窗的显示
      deleteIndex: null, // 当前要删除的项目索引
    };
  },
  computed: {
    totalPages() {
      return Math.ceil(this.totalProjects / this.pageSize);
    },
  },
  mounted() {
    this.fetchProjects();
  },
  methods: {
    // 打开删除确认弹窗
    openDeleteConfirmation(index) {
      this.deleteIndex = index;
      this.isDeleteConfirmationOpen = true;
    },
    // 关闭删除确认弹窗
    closeDeleteConfirmation() {
      this.isDeleteConfirmationOpen = false;
      this.deleteIndex = null;
    },
    // 确认删除
    async confirmDelete() {
      if (this.deleteIndex !== null) {
        await this.deleteProject(this.deleteIndex);
        this.closeDeleteConfirmation();
      }
    },
    // 删除项目
    async deleteProject(index) {
      try {
        const projectId = this.projects[index].id;
        const response = await this.$rustInvoke('delete_project', {
          projectId: projectId,
        });
        console.log(response);

        if (response.valid) {
          await this.fetchProjects();
          this.$message.success('删除成功！');
        } else {
          console.error('删除项目失败:', response.message);
          this.$message.error('删除失败：' + response.message);
        }
      } catch (error) {
        console.error('调用删除项目接口失败:', error);
        this.$message.error('删除失败，请重试！');
      }
    },
    // 其他方法保持不变
    openDrawer() {
      this.isDrawerOpen = true;
    },
    closeDrawer() {
      this.isDrawerOpen = false;
      this.editedProject = {
        index: null,
        name: '',
        description: '',
      };
    },
    viewRelatedReportList(projectNumber) {
      this.projectNumber = projectNumber;
      this.openDialog();
    },
    async handleSave(project) {
      try {
        const response = await this.$rustInvoke('save_project', {
          projectName: project.name,
          description: project.description,
        });
        console.log(response);

        if (response.valid) {
          await this.fetchProjects();
          this.closeDrawer();
        } else {
          console.error('保存项目失败:', response.message);
        }
      } catch (error) {
        console.error('调用保存项目接口失败:', error);
      }
    },
    async fetchProjects() {
      try {
        const response = await this.$rustInvoke('fetch_project_list', {
          currentPage: this.currentPage,
          pageSize: this.pageSize,
          keyword: this.searchKeyword,
        });
        console.log(response);

        if (response.valid) {
          this.projects = response.projects;
          this.totalProjects = response.total;
        } else {
          console.error('获取项目列表失败:', response.message);
        }
      } catch (error) {
        console.error('调用获取项目列表接口失败:', error);
      }
    },
    prevPage() {
      if (this.currentPage > 1) {
        this.currentPage--;
        this.fetchProjects();
      }
    },
    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.currentPage++;
        this.fetchProjects();
      }
    },
    openDialog() {
      this.isDialogOpen = true;
    },
    closeDialog() {
      this.isDialogOpen = false;
    },
    openSiteMaintenance(projectNumber) {
      this.projectNumber = projectNumber;
      this.isSiteDialogOpen = true;
    },
    closeSiteDialog() {
      this.isSiteDialogOpen = false;
      this.projectNumber = '';
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>