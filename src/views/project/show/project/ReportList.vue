<template>
  <div>
    <!-- 数据列表 -->
    <div v-if="filteredDataList.length > 0" style="max-height: 400px;overflow: auto;">
      <table class="min-w-full divide-y divide-gray-200 w-full text-left">
        <thead class="bg-gray-50">
        <tr>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">项目</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">报告编号</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">源文件</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">日期</th>
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
        </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
        <tr v-for="(item, index) in filteredDataList" :key="index">
          <td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500">{{ item.projectName }}</td>
          <td class="px-6 py-4 text-sm text-gray-500">{{ item.reportNumber }}</td>
          <td class="px-6 py-4 text-sm whitespace-nowrap text-gray-500 text-left">
            <!-- 遍历 sourceFiles 数组 -->
            <div v-for="(file, fileIndex) in item.sourceFiles" :key="fileIndex" class="block mb-1">
                <span
                    @click="viewData(file, item.reportNumber)"
                    class="cursor-pointer px-2 py-1 bg-blue-100 text-blue-800 text-xs font-medium rounded-full hover:bg-blue-200"
                >
                  {{ file }}
                </span>
            </div>
          </td>
          <td class="py-4 whitespace-nowrap text-sm text-gray-500">{{ item.createTime }}</td>
          <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
            <button
                v-if="item.state === 2"
                @click="viewItem(item)"
                class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-orange-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              查看
            </button>


            <button
                v-if="item.state !== 2"
                @click="analyzeData(item)"
                class="px-3 py-1 text-sm font-medium text-white bg-orange-500 rounded-md hover:bg-orange-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              开始分析
            </button>
            <button
                @click="deleteItem(item.reportNumber)"
                class="ml-1 px-3 py-1 text-sm font-medium text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-orange-500"
            >
              删除
            </button>
          </td>
        </tr>
        </tbody>
      </table>

      <!-- 分页控件 -->
      <div class="mt-1">
        <Pagination
            v-if="filteredDataList.length > 0"
            :currentPage="currentPage"
            :totalPages="totalPages"
            @update:currentPage="handlePageChange"
        />
      </div>
    </div>

    <!-- 无数据时的提示 -->
    <div v-else class="text-center py-6 text-gray-500">
      暂无数据，请去导入项目相关的表格进行数据分析。
    </div>

    <!-- 查看数据的 Dialog -->
    <SlotDialog :isOpen="isDialogOpen" :showConfirm="false" title="报告详情" @close="closeDialog">
      <SummaryView :reportNumber="reportNumber" />
    </SlotDialog>

  </div>
</template>

<script>
import SlotDialog from '@/components/SlotDialog.vue';
import SummaryView from '@/views/project/show/dashboard/SummaryView.vue';
import Pagination from "@/components/PaginationView.vue";

export default {
  name: 'ReportList',
  components: {
    Pagination,
    SlotDialog,
    SummaryView,
  },
  props: {
    projectNumber: {
      type: String,
      default: '',
    },
    pageSize: {
      type: Number,
      default: 10,
    },
  },
  data() {
    return {
      dataList: [],
      currentPage: 1,
      totalPages: 1,
      isDialogOpen: false,
      isSourceFileDialogOpen: false,
      reportNumber: '',
      sourceFileDetail: [],
      sourceFileHeaders: [],
      isSourceFileDataLoaded: false, // 数据是否加载完成
      sourceFileLoadCurrentPage: 1, // 数据是否加载完成
      // header表头
      sourceFileHeadersMap: [
        {
          project_name: 'Project Name',
          site_name: 'Site Name',
          site_number: 'Site Number',
          subject_id: 'Subject ID',
          instance_name: 'Instance Name',
          data_page_name: 'Data Page Name',
          days_of_missing_pages: 'Days of missing Pages',
          // md_days: 'Days',
          md_gt7: 'MP ＞ 7days',
          md_gt14: 'MP > 14days',
        },
        {
          study: 'Study',
          site_name: 'SiteName',
          study_environment_site: 'StudyEnvironmentSiteNumber',
          subject_name: 'SubjectName',
          folder: 'Folder',
          form: 'Form',
          log_id: 'Log#',
          qry_open_date: 'QryOpenDate',
          qry_open_date_localized: 'QryOpenDateLocalized',
          op_days: 'QryOpenDays',
          op_gt7: 'Open>7days',
          op_gt14: 'Open>14days',
          op_gt21: 'Open＞21days',
          op_gt30: 'Open≥30days',
          qry_open_by: 'QryOpenBy',
          query_text: 'QueryText',
          marking_group_name: 'MarkingGroupName',
          qry_response_date: 'QryResponseDate',
          qry_response_date_localized: 'QryResponseDateLocalized',
          qry_response_days: 'QryResponseDays',
          qry_response_user: 'QryRespond',
          qry_answer: 'AnswerText',
          qry_status: 'Name',
        },
        {
          study: 'Study',
          site: 'Site',
          subject: 'Subject',
          folder_name: 'Folder Name',
          page: 'Page',
          entered: 'Entered',
          verify_required: 'Verify Required',
        }
      ],
      sourceFileName: '',
      isSourceFilDataLoading: false
    };
  },
  computed: {
    filteredDataList() {
      return this.dataList;
    },
  },
  watch: {
    projectNumber() {
      this.currentPage = 1;
      this.fetchData();
    },
    currentPage() {
      this.fetchData();
    },
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    handlePageChange(newPage) {
      this.currentPage = newPage;
      this.fetchData();
    },
    async fetchData() {
      //this.$rustInvoke("process_message");
      await this.$rustListen('listen_message', (event) => {
        console.log(event)
      });
      try {
        const response = await this.$rustInvoke('fetch_report_list', {
          keyword: this.projectNumber,
          currentPage: this.currentPage,
          pageSize: this.pageSize,
        });
        this.dataList = response.data || [];
        this.totalPages = Math.ceil(response.total / this.pageSize);
      } catch (error) {
        console.error('获取报告列表失败:', error);
        this.dataList = [];
        this.totalPages = 1;
      }
    },
    viewItem(item) {
      this.reportNumber = item.reportNumber;
      this.isDialogOpen = true;
    },
    closeDialog() {
      this.isDialogOpen = false;
    },
    async deleteItem(reportNumber) {
      let result = await confirm('确定删除该项目吗？');
      if (result) {
        try {
          const response = await this.$rustInvoke('delete_report_item', {
            reportNumber: reportNumber,
          });
          if (response.valid) {
            await this.fetchData()
          } else {
            this.$showModal('删除失败，请重试！');
          }
        } catch (error) {
          console.error('删除项目失败:', error);
        }
      }
    },
    prevPage() {
      if (this.currentPage > 1) {
        this.currentPage -= 1;
      }
    },
    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.currentPage += 1;
      }
    },
    async analyzeData(item) {
      const closeModal = this.$showModal('系统正在分析数据，请稍候...', {
        showCloseButton: false,
        allowOutsideClick: false,
      });
      try {
        const result = await this.$rustInvoke('analyze_report_data', {
          projectNumber: item.projectName,
          reportNumber: item.reportNumber,
        });
        if (result.valid) {
          item.state = result.state;
          this.$showModal('数据分析完成！');
          await this.fetchData();
        } else {
          this.$showModal('数据分析失败，请重试！');
        }
      } catch (error) {
        this.$showModal('网络异常，请稍后重试！');
      }
      closeModal();
    },
    async viewData(sourceFileName, reportNumber) {

      const closeModal = this.$showModal("正在拉取数据数据，请稍后....", {
        showCloseButton: false,
      });
      try {
        if (sourceFileName.toLocaleLowerCase().indexOf("missing") > -1) {
          this.sourceFileHeaders = this.sourceFileHeadersMap[0];
        } else if (sourceFileName.toLocaleLowerCase().indexOf("query") > -1) {
          this.sourceFileHeaders = this.sourceFileHeadersMap[1];
        } else {
          this.sourceFileHeaders = this.sourceFileHeadersMap[2];
        }
        console.log(this.sourceFileName.toLocaleLowerCase(), this.sourceFileHeaders)
        const response = await this.$rustInvoke('fetch_origin_detail', {
          sourceFileName: sourceFileName,
          reportNumber: reportNumber,
          currentPage: this.sourceFileLoadCurrentPage,
          tableHeader: Object.keys(this.sourceFileHeaders),
          tableHeaderMap: this.sourceFileHeaders,
        });
        if (response.valid) {
          closeModal();
        } else {
          this.$showModal('获取数据失败，请重试！');
          closeModal();
        }
      } catch (error) {
        console.error('获取源文件详情失败:', error);
        closeModal();
      }
    },
    closeSourceFileDialog() {
      this.isSourceFileDialogOpen = false;
      this.sourceFileDetail = [];
    },
  },
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>
