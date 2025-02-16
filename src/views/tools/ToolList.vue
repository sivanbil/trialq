<template>
  <div class="min-h-screen bg-gray-50 flex flex-col">
    <!-- 头部 -->
    <HeaderView />

    <!-- 内容区域 -->
    <div class="flex-1 overflow-y-auto p-4">
      <div class="bg-white p-6 rounded-lg shadow-md text-left mx-auto">
        <!-- 工具区块按钮 -->
        <div class="flex space-x-4 mb-6">
          <!-- 常用工具按钮 -->
          <button
              @click="openDefaultToolsDialog"
              class="px-4 py-2 bg-purple-800 text-white rounded-lg hover:bg-purple-700 transition-colors"
          >
            默认工具
          </button>

          <!-- 我的工具按钮 -->
          <button
              @click="openUserToolsDialog"
              class="px-4 py-2 bg-purple-800 text-white rounded-lg hover:bg-purple-700 transition-colors"
          >
            我的工具
          </button>
        </div>
      </div>
    </div>

    <!-- 底部 -->
    <FooterView />

    <!-- 常用工具弹窗 -->
    <SlotDialog :isOpen="isDefaultToolsDialogOpen" title="常用工具" @close="closeDefaultToolsDialog">
      <div class="space-y-3">
        <div
            v-for="(tool, index) in defaultTools"
            :key="index"
            class="flex items-center justify-between p-4 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow"
        >
          <a
              :href="tool.link_url"
              target="_blank"
              class="text-left text-purple-800 font-medium hover:text-purple-700 transition-colors"
          >
            {{ tool.name }}
            <p>{{tool.link_url}}</p>
          </a>
          <!-- 默认工具不允许删除 -->
          <span class="text-sm text-gray-500">默认工具</span>
        </div>
      </div>
    </SlotDialog>

    <!-- 我的工具弹窗 -->
    <SlotDialog :isOpen="isUserToolsDialogOpen" title="我的工具" @close="closeUserToolsDialog">
      <div>
        <!-- 添加工具按钮 -->
        <div class="mb-6 text-right">
          <button
              @click="openDrawer"
              class="px-4 py-2 bg-purple-800 text-white rounded-lg hover:bg-purple-700 transition-colors"
          >
            添加我常用的工具
          </button>
        </div>

        <!-- 我的工具列表 -->
        <div v-if="userTools.length === 0" class="text-gray-500">
          暂无工具，点击右上角按钮添加。
        </div>
        <div v-else class="space-y-3">
          <div
              v-for="(tool, index) in userTools"
              :key="index"
              class="text-left flex items-center justify-between p-4 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow"
          >
            <a
                :href="tool.link_url"
                target="_blank"
                class="text-purple-800 font-medium hover:text-purple-700 transition-colors"
            >
              {{ tool.name }}
              <p>{{tool.link_url}}</p>
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
          <!-- 分页控件 -->
          <div class="mt-1">
            <!-- 分页控件 -->
            <Pagination
                v-if="userTools.length > 0"
                :currentPage="currentPage"
                :totalPages="totalUserPages"
                @update:currentPage="handlePageChange"
            />
          </div>

        </div>
      </div>
    </SlotDialog>

    <!-- 抽屉表单 -->
    <ToolFormDrawer :isOpen="isDrawerOpen" @close="closeDrawer" @save="handleSave" />
  </div>
</template>

<script>
import ToolFormDrawer from './ToolDrawerForm.vue'; // 引入抽屉表单组件
import SlotDialog from '@/components/SlotDialog.vue';
import Pagination from "@/components/PaginationView.vue"; // 引入弹窗组件

export default {
  name: 'ClinicalTools',
  components: {
    Pagination,
    ToolFormDrawer,
    SlotDialog,
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
      totalUserPages: 1,
      // 弹窗相关状态
      isDefaultToolsDialogOpen: false, // 常用工具弹窗
      isUserToolsDialogOpen: false, // 我的工具弹窗
      // 抽屉相关状态
      isDrawerOpen: false,
    };
  },
  mounted() {
    // 从后端获取用户工具列表
    this.fetchTools();
  },
  methods: {
    // 处理页码变化
    handlePageChange(newPage) {
      this.currentPage = newPage;
      this.fetchTools();
    },
    // 打开常用工具弹窗
    openDefaultToolsDialog() {
      this.isDefaultToolsDialogOpen = true;
    },
    // 关闭常用工具弹窗
    closeDefaultToolsDialog() {
      this.isDefaultToolsDialogOpen = false;
    },
    // 打开我的工具弹窗
    openUserToolsDialog() {
      this.isUserToolsDialogOpen = true;
    },
    // 关闭我的工具弹窗
    closeUserToolsDialog() {
      this.isUserToolsDialogOpen = false;
    },
    // 打开抽屉
    openDrawer() {
      this.isDrawerOpen = true;
      this.closeUserToolsDialog();
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
          await this.fetchTools();

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
          this.totalUserPages = Math.ceil(response.total / this.itemsPerPage);
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
          await this.fetchTools();
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
