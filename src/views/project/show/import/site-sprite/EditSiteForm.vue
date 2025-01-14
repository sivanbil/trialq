<template>
  <form @submit.prevent="handleSubmit"> <!-- 阻止默认行为 -->
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
          @click="$emit('close')"
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
</template>

<script>
export default {
  name: 'EditSiteForm',
  props: {
    initialData: {
      type: Object,
      required: true,
    },
  },
  data() {
    return {
      formData: {...this.initialData},
    };
  },
  methods: {
    handleSubmit() {
      this.$emit('submit', this.formData); // 触发 submit 事件
    },
  },
};
</script>