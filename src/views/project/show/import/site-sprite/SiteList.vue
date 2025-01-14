<template>
  <div>
    <!-- 中心列表 -->
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
              @click="$emit('edit', center)"
              class="px-2 py-1 text-sm text-blue-500 hover:text-blue-700"
          >
            编辑
          </button>
          <button
              @click="$emit('delete', center.id)"
              class="px-2 py-1 text-sm text-red-500 hover:text-red-700"
          >
            删除
          </button>
        </td>
      </tr>
      </tbody>
    </table>

    <!-- 分页控件 -->
    <div class="mt-4 flex justify-between items-center">
      <div class="text-sm text-gray-700">
        当前第 {{ currentPage }} 页，共 {{ totalPages }} 页
      </div>
      <div class="flex space-x-2">
        <button
            @click="$emit('prev-page')"
            :disabled="currentPage === 1"
            class="px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50"
        >
          上一页
        </button>
        <button
            @click="$emit('next-page')"
            :disabled="currentPage === totalPages"
            class="px-4 py-2 bg-gray-200 rounded-md hover:bg-gray-300 disabled:opacity-50"
        >
          下一页
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'SiteList',
  props: {
    centers: {
      type: Array,
      required: true,
    },
    currentPage: {
      type: Number,
      required: true,
    },
    totalPages: {
      type: Number,
      required: true,
    },
  },
};
</script>