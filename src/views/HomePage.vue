<template>

  <CoolBanner />
  <div class="metro-container">

    <div class="app-container">
      <!-- Metro 布局 -->
      <div class="metro-dashboard">
        <!-- 遮盖层 -->
        <div v-if="!hasLicence" class="overlay">
          <p class="overlay-text">请输入注册码以解锁功能</p>
          <button @click="showLicenseModal = true">输入注册码</button>
        </div>

        <!-- 大磁贴（左边） -->
        <div
            class="tile large"
            :style="{ backgroundColor: '#ff8c00', pointerEvents: hasLicence ? 'auto' : 'none' }"
            @click="goToProjectManagement"
        >
          <h1>项目管理</h1>
        </div>

        <!-- 小磁贴（右上方） -->
        <div
            class="tile small"
            :style="{ backgroundColor: '#4b0082', pointerEvents: hasLicence ? 'auto' : 'none' }"
            @click="goToToolList"
        >
          <p>常用工具</p>
        </div>

        <!-- 小磁贴（右下方） -->
        <div
            class="tile small"
            :style="{ backgroundColor: '#00bcf2', pointerEvents: hasLicence ? 'auto' : 'none' }"
            @click="goToUserSpace"
        >
          <p>个人设置</p>
        </div>
      </div>

      <!-- 许可证弹窗组件 -->
      <LicenseModal
          :visible="showLicenseModal"
          @close="showLicenseModal = false"
          @save="handleSaveLicense"
      />
    </div>
  </div>

</template>

<script>
import LicenseModal from '../components/LicenseModal.vue'; // 引入许可证弹窗组件

export default {
  name: 'HomePage',
  components: {
    LicenseModal, // 注册组件
  },
  data() {
    return {
      hasLicence: false, // 默认没有许可证
      showLicenseModal: false, // 控制弹窗显示
    };
  },
  methods: {
    goToProjectManagement() {
      if (this.hasLicence) {
        this.$router.push('/project/management');
      }
    },
    goToUserSpace() {
      if (this.hasLicence) {
        this.$router.push('/user/space');
      }
    },
    goToToolList() {
      if (this.hasLicence) {
        this.$router.push('/tool/list');
      }
    },
    handleSaveLicense(formValue) {
      let licenseKey = formValue.licenseKey;
      let email = formValue.email;
      if (licenseKey) {
        // 保存到 localStorage
        localStorage.setItem('licenseKey', licenseKey);
        this.hasLicence = true; // 更新许可证状态

        // 调用 Rust WebAssembly 函数发送请求
        this.sendLicenseToBackend(email, licenseKey);
      }
    },
    async sendLicenseToBackend(email,licenseKey) {
      if (email && licenseKey) {
        // 保存到 localStorage
        localStorage.setItem('email', email);
        localStorage.setItem('licenseKey', licenseKey);
        this.hasLicence = true; // 更新许可证状态
        // 调用 Tauri 命令发送许可证到后端
        try {
          const response = await this.$rustInvoke('send_license', {
            email: email,
            licenseKey: licenseKey,
          });
          console.log('License validation response:', response);

          if (response.valid) {
            alert('许可证验证成功！');
          } else {
            alert('许可证无效：' + response.message);
          }
        } catch (error) {
          console.error('Failed to validate license:', error);
          alert('许可证验证失败：' + error);
        }
      }
    },
  },
  mounted() {
    // 检查 localStorage 中是否有许可证
    const savedLicenseKey = localStorage.getItem('licenseKey');
    if (savedLicenseKey) {
      this.hasLicence = true;
    }
  },
};
</script>

<style scoped>

</style>