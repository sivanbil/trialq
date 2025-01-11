<template>
  <div class="min-h-screen bg-gray-50">
    <HeaderView />
    <div class="mx-auto p-4">
      <div class="bg-white p-2 rounded-lg shadow-md text-left">
        <!-- 添加工具按钮 -->
        <div class="mb-6 text-right">
          <button
              @click="openDrawer"
              class="px-4 py-2 bg-purple-800 text-white rounded-lg hover:bg-purple-700 transition-colors"
          >
            添加我常用的工具
          </button>
        </div>

        <!-- 默认工具区块 -->
        <div class="mb-8">
          <h2 class="text-lg font-semibold mb-4">默认工具</h2>
          <div class="space-y-3">
            <div
                v-for="(tool, index) in defaultTools"
                :key="index"
                class="flex items-center justify-between p-4 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow"
            >
              <a
                  :href="tool.url"
                  target="_blank"
                  class="text-purple-800 font-medium hover:text-purple-700 transition-colors"
              >
                {{ tool.name }}
              </a>
              <!-- 默认工具不允许删除 -->
              <span class="text-sm text-gray-500">默认工具</span>
            </div>
          </div>
        </div>

        <!-- 用户工具区块 -->
        <div>
          <h2 class="text-lg font-semibold mb-4">我的工具</h2>
          <div v-if="userTools.length === 0" class="text-gray-500">
            暂无工具，点击右上角按钮添加。
          </div>
          <div v-else class="space-y-3">
            <div
                v-for="(tool, index) in paginatedUserTools"
                :key="index"
                class="flex items-center justify-between p-4 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow"
            >
              <a
                  :href="tool.link_url"
                  target="_blank"
                  class="text-purple-800 font-medium hover:text-purple-700 transition-colors"
              >
                {{ tool.name }}
              </a>
              <!-- 用户工具允许删除 -->
              <button
                  @click="removeTool(index)"
                  class="px-3 py-1 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors"
              >
                删除
              </button>
            </div>

            <!-- 分页控件 -->
            <div v-if="userTools.length > itemsPerPage" class="flex items-center justify-center gap-3 mt-6">
              <button
                  @click="currentPage = 1"
                  :disabled="currentPage === 1"
                  class="px-3 py-1 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                首页
              </button>
              <button
                  @click="currentPage--"
                  :disabled="currentPage === 1"
                  class="px-3 py-1 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                上一页
              </button>
              <span class="text-sm text-gray-600">第 {{ currentPage }} 页 / 共 {{ totalUserPages }} 页</span>
              <button
                  @click="currentPage++"
                  :disabled="currentPage === totalUserPages"
                  class="px-3 py-1 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                下一页
              </button>
              <button
                  @click="currentPage = totalUserPages"
                  :disabled="currentPage === totalUserPages"
                  class="px-3 py-1 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                末页
              </button>
            </div>
          </div>
        </div>

        <!-- 抽屉表单 -->
        <ToolFormDrawer :isOpen="isDrawerOpen" @close="closeDrawer" @save="handleSave" />
      </div>
    </div>
    <FooterView />
  </div>
</template>

<script>
import ToolFormDrawer from './ToolDrawerForm.vue'; // 引入抽屉表单组件

export default {
  name: 'ClinicalTools',
  components: {
    ToolFormDrawer,
  },
  data() {
    return {
      // 默认工具列表
      defaultTools: [
        { name: '药品审评中心', link_url: 'https://www.cde.org.cn' },
        { name: '国家卫生健康委员会', link_url: 'http://www.nhc.gov.cn' },
      ],
      // 用户工具列表
      userTools: [],
      // 分页相关
      currentPage: 1, // 当前页码
      itemsPerPage: 5, // 每页显示的工具数量
      // 抽屉相关状态
      isDrawerOpen: false,
    };
  },
  computed: {
    // 当前页的用户工具列表
    paginatedUserTools() {
      const start = (this.currentPage - 1) * this.itemsPerPage;
      const end = start + this.itemsPerPage;
      return this.userTools.slice(start, end);
    },
    // 用户工具的总页数
    totalUserPages() {
      return Math.ceil(this.userTools.length / this.itemsPerPage);
    },
  },
  mounted() {
    // 从后端获取用户工具列表
    this.fetchTools();
  },
  methods: {
    // 打开抽屉
    openDrawer() {
      this.isDrawerOpen = true;
    },
    // 关闭抽屉
    closeDrawer() {
      this.isDrawerOpen = false;
    },
    // 处理保存
    handleSave(newTool) {
      this.saveTools(newTool); // 保存到后端
      this.closeDrawer();
    },
    // 删除工具
    async removeTool(index) {
      const globalIndex = (this.currentPage - 1) * this.itemsPerPage + index;
      const toolToDelete = this.userTools[globalIndex];

      try {
        // 调用后端的 delete_tool 方法
        const response = await this.$rustInvoke('delete_tool', {
          toolId: toolToDelete.id, // 假设每个工具都有一个唯一的 id
        });

        if (response.valid) {
          // 从前端的 userTools 列表中移除工具
          this.userTools.splice(globalIndex, 1);

          // 如果删除后当前页没有数据，且不是第一页，则跳转到上一页
          if (this.paginatedUserTools.length === 0 && this.currentPage > 1) {
            this.currentPage--;
          }

          this.$showModal('删除成功');
        } else {
          this.$showModal('删除失败');
        }
      } catch (error) {
        console.error('Failed to delete tool:', error);
        this.$showModal('删除失败');
      }
    },
    // 从后端获取用户工具列表
    async fetchTools() {
      try {
        const response = await this.$rustInvoke('fetch_tool_list', {
          currentPage: this.currentPage,
          pageSize: this.itemsPerPage,
        });
        console.log('Tool list response:', response);

        if (response.valid) {
          this.userTools = response.tools; // 更新用户工具列表
        }
      } catch (error) {
        console.error('Failed to fetch tools:', error);
      }
    },
    // 保存用户工具列表到后端
    async saveTools(new_tool) {
      try {
        const response = await this.$rustInvoke('save_tool', {
          title: new_tool.name,
          linkUrl: new_tool.url,
        });
        console.log(response);
        if (response.valid) {

          this.userTools.push(response.tool); // 添加到用户工具列表
          this.$showModal('存储成功');
        } else {
          this.$showModal('存储失败');
        }
      } catch (error) {
        this.$showModal('存储失败.error:', error);
      }
    },

  },
};
</script>