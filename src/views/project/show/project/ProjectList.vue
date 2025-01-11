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
      <ul class="space-y-4">
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
              >
                查看相关报告数据
              </button>

              <button
                  @click="deleteProject(index)"
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
  </div>
</template>

<script>
import ProjectFormDrawer from './ProjectDrawerForm.vue'; // 引入抽屉表单组件

export default {
  name: 'ProjectList',
  components: {
    ProjectFormDrawer,
  },
  data() {
    return {
      // 项目列表
      projects: [],
      // 分页相关
      currentPage: 1,
      pageSize: 10,
      totalProjects: 0,
      // 搜索关键字
      searchKeyword: '',
      // 抽屉相关状态
      isDrawerOpen: false,
      // 编辑项目相关数据
      editedProject: {
        index: null,
        name: '',
        description: '',
      },
    };
  },
  computed: {
    // 计算总页数
    totalPages() {
      return Math.ceil(this.totalProjects / this.pageSize);
    },
  },
  mounted() {
    // 初始化时获取项目列表
    this.fetchProjects();
  },
  methods: {
    // 打开抽屉
    openDrawer() {
      this.isDrawerOpen = true;
    },
    // 关闭抽屉
    closeDrawer() {
      this.isDrawerOpen = false;
      this.editedProject = {
        index: null,
        name: '',
        description: '',
      };
    },
    // 处理保存
    async handleSave(project) {
      try {
        const response = await this.$rustInvoke('save_project', {
          projectName: project.name,
          description: project.description,
        });
        console.log(response);

        if (response.valid) {
          // 保存成功，刷新项目列表
          this.fetchProjects();
          this.closeDrawer();
        } else {
          console.error('保存项目失败:', response.message);
        }
      } catch (error) {
        console.error('调用保存项目接口失败:', error);
      }
    },
    // 删除项目
    async deleteProject(index) {
      if (confirm('确定删除该项目吗？')) {
        try {
          const projectId = this.projects[index].id; // 假设项目对象中有 id 字段
          const response = await this.$rustInvoke('delete_project', {
            projectId: projectId,
          });
          console.log(response);

          if (response.valid) {
            // 删除成功，刷新项目列表
            await this.fetchProjects();
          } else {
            console.error('删除项目失败:', response.message);
          }
        } catch (error) {
          console.error('调用删除项目接口失败:', error);
        }
      }
    },
    // 获取项目列表
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
    // 上一页
    prevPage() {
      if (this.currentPage > 1) {
        this.currentPage--;
        this.fetchProjects();
      }
    },
    // 下一页
    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.currentPage++;
        this.fetchProjects();
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>