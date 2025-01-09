<template>
  <div class="min-h-screen bg-gray-50">
    <HeaderView/>
    <div class="mx-auto bg-white p-6 rounded-lg shadow-md">

      <!-- 常用设置 -->
      <div class="mb-6 p-4 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow">
        <h2 class="text-xl font-semibold text-purple-800 mb-3">常用设置</h2>

        <!-- 注册码输入 -->
        <div class="mb-4">
          <textarea
              v-model="registrationCode"
              placeholder="请输入注册码"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
              rows="3"
          ></textarea>
        </div>

        <!-- 保存设置按钮 -->
        <button
            @click="saveSettings"
            class="w-full bg-purple-800 text-white py-2 px-4 rounded-md hover:bg-purple-700 transition-colors"
        >
          保存设置
        </button>
      </div>
    </div>
    <FooterView/>
  </div>
</template>

<script>

export default {
  name: 'UserSettings',
  data() {
    return {
      // 常用设置
      autoUpdateClient: false,
      syncDataToCloud: false,
    };
  },
  methods: {
    // 保存常用设置
    saveSettings() {
      const settings = {
        autoUpdateClient: this.autoUpdateClient,
        syncDataToCloud: this.syncDataToCloud,
      };
      localStorage.setItem('userSettings', JSON.stringify(settings));
      alert('设置保存成功');
    },
  },
};
</script>