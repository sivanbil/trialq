<template>
  <div class="bg-white p-6 rounded-lg shadow-md text-left">

    <!-- 添加项目按钮 -->
    <div class="mb-6 text-right">
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
              <h3 class="text-lg font-semibold text-purple-800">{{ project.name }}</h3>
              <p class="text-sm text-gray-600">{{ project.description }}</p>
            </div>
            <div class="flex space-x-2">
              <button
                  @click="editProject(index)"
                  class="px-3 py-1 text-sm font-medium text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                查看
              </button>
              <button
                  @click="editProject(index)"
                  class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                编辑
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
      projects: [
        { name: '项目A', description: '这是项目A的描述' },
        { name: '项目B', description: '这是项目B的描述' },
      ],
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
    handleSave(project) {
      if (project.index !== null) {
        // 更新项目
        this.projects[project.index] = {
          name: project.name,
          description: project.description,
        };
      } else {
        // 添加新项目
        this.projects.push({
          name: project.name,
          description: project.description,
        });
      }
      this.closeDrawer();
    },
    // 编辑项目
    editProject(index) {
      this.editedProject = {
        index,
        name: this.projects[index].name,
        description: this.projects[index].description,
      };
      this.openDrawer();
    },
    // 删除项目
    deleteProject(index) {
      if (confirm('确定删除该项目吗？')) {
        this.projects.splice(index, 1);
      }
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>