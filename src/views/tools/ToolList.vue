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

        <!-- 工具列表 -->
        <div class="space-y-3 overflow-y-auto text-left" style="max-height: 64vh">
          <h2 class="text-lg font-semibold mb-4">临床试验工具列表</h2>
          <div
              v-for="(tool, index) in paginatedTools"
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
            <button
                @click="removeTool(index)"
                class="px-3 py-1 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors"
            >
              删除
            </button>
          </div>
        </div>

        <!-- 分页控件 -->
        <div class="flex items-center justify-center gap-3 mt-6">
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
          <span class="text-sm text-gray-600">第 {{ currentPage }} 页 / 共 {{ totalPages }} 页</span>
          <button
              @click="currentPage++"
              :disabled="currentPage === totalPages"
              class="px-3 py-1 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            下一页
          </button>
          <button
              @click="currentPage = totalPages"
              :disabled="currentPage === totalPages"
              class="px-3 py-1 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            末页
          </button>
        </div>

        <!-- 抽屉表单 -->
        <ToolFormDrawer
            :isOpen="isDrawerOpen"
            @close="closeDrawer"
            @save="handleSave"
        />
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
      tools: [
        { name: '临床试验注册平台', url: 'https://clinicaltrials.gov', frequency: '高' },
        { name: 'Medline 文献检索', url: 'https://pubmed.ncbi.nlm.nih.gov', frequency: '高' },
        { name: '数据统计分析工具', url: 'https://www.r-project.org', frequency: '中' },
        { name: '中国临床试验注册中心', url: 'http://www.chictr.org.cn', frequency: '高' },
        { name: '药品审评中心', url: 'https://www.cde.org.cn', frequency: '高' },
        { name: '中国知网', url: 'https://www.cnki.net', frequency: '中' },
        { name: '万方数据', url: 'https://www.wanfangdata.com.cn', frequency: '中' },
        { name: '中国生物医学文献数据库', url: 'https://www.sinomed.ac.cn', frequency: '中' },
        { name: '国家卫生健康委员会', url: 'http://www.nhc.gov.cn', frequency: '中' },
        { name: '医学信息研究所', url: 'http://www.imicams.ac.cn', frequency: '低' },
      ],
      // 分页相关
      currentPage: 1, // 当前页码
      itemsPerPage: 5, // 每页显示的工具数量
      // 抽屉相关状态
      isDrawerOpen: false,
    };
  },
  computed: {
    // 当前页的工具列表
    paginatedTools() {
      const start = (this.currentPage - 1) * this.itemsPerPage;
      const end = start + this.itemsPerPage;
      return this.tools.slice(start, end);
    },
    // 总页数
    totalPages() {
      return Math.ceil(this.tools.length / this.itemsPerPage);
    },
  },
  mounted() {
    // 从后端获取工具列表
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
      this.tools.push(newTool);
      this.saveTools(); // 保存到后端
      this.closeDrawer();
    },
    // 删除工具
    removeTool(index) {
      const globalIndex = (this.currentPage - 1) * this.itemsPerPage + index;
      this.tools.splice(globalIndex, 1);
      this.saveTools(); // 保存到后端
    },
    // 从后端获取工具列表
    async fetchTools() {
      try {
        const response = await fetch('/api/tools'); // 假设后端接口是 /api/tools
        const data = await response.json();
        if (data && data.tools) {
          this.tools = data.tools;
        }
      } catch (error) {
        console.error('Failed to fetch tools:', error);
      }
    },
    // 保存工具列表到后端
    async saveTools() {
      try {
        await fetch('/api/tools', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ tools: this.tools }),
        });
      } catch (error) {
        console.error('Failed to save tools:', error);
      }
    },
  },
};
</script>