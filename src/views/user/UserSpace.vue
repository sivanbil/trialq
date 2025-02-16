<template>
  <div class="min-h-screen bg-gray-50">
    <HeaderView/>
    <div class="mx-auto bg-white p-6 rounded-lg shadow-md">

      <!-- 常用设置 -->
      <div class="mb-6 p-4 bg-white border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow">
        <h2 class="text-xl font-semibold text-purple-800 mb-3">常用设置</h2>

        <!-- 邮箱输入 -->
        <div class="mb-4">
          <input
              v-model="email"
              :readonly="true"
              :disabled="true"
              placeholder="请输入邮箱"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
          />
        </div>

        <!-- 注册码输入 -->
        <div class="mb-4">
          <textarea
              v-model="registrationCode"
              placeholder="请输入注册码"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none text-green-500 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
              rows="6"
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
      // 输入框绑定的数据
      email: '',
      registrationCode: '',

      // 从 localStorage 中读取的已保存数据
      savedEmail: '',
      savedRegistrationCode: '',
    };
  },
  methods: {
    // 保存设置
    saveSettings() {
      // 检查注册码是否为空
      if (!this.registrationCode.trim()) {
        this.$showModal('注册码不能为空');
        return;
      }

      // 检查注册码是否与本地注册码相同
      const localLicenseKey = localStorage.getItem('licenseKey');
      if (this.registrationCode === localLicenseKey) {
        this.$showModal('注册码与本地注册码相同，无需保存');
        return;
      }

      // 保存到 localStorage
      localStorage.setItem('email', this.email);
      localStorage.setItem('licenseKey', this.registrationCode);

      // 更新已保存的数据
      this.savedEmail = this.email;
      this.savedRegistrationCode = this.registrationCode;

      alert('设置保存成功');
    },

    // 从 localStorage 中加载已保存的数据
    loadSavedSettings() {
      this.email = localStorage.getItem('email') || '';
      this.registrationCode = localStorage.getItem('licenseKey') || '';

      // 如果本地没有注册码，跳转到首页
      if (!this.registrationCode) {
        this.$router.push('/');  // 假设首页路由为 '/'
      }
    },
  },
  mounted() {
    // 组件加载时，从 localStorage 中加载已保存的数据
    this.loadSavedSettings();
  },
};
</script>
