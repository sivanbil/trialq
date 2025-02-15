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
            :style="{ backgroundColor: '#ff8000', pointerEvents: hasLicence ? 'auto' : 'none' }"
            @click="goToProjectManagement"
        >
          <h1>项目管理</h1>
        </div>

        <!-- 小磁贴（右上方） -->
        <div
            class="tile small"
            :style="{ backgroundColor: '#8000FF', pointerEvents: hasLicence ? 'auto' : 'none' }"
            @click="goToToolList"
        >
          <p>常用工具</p>
        </div>

        <!-- 小磁贴（右下方） -->
        <div
            class="tile small"
            :style="{ backgroundColor: '#0080FF', pointerEvents: hasLicence ? 'auto' : 'none' }"
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

        // 调用 Rust WebAssembly 函数发送请求
        this.sendLicenseToBackend(email, licenseKey);
      }
    },
    async sendLicenseToBackend(email,licenseKey) {
      if (email && licenseKey) {

        // 调用 Tauri 命令发送许可证到后端
        try {
          const response = await this.$rustInvoke('send_license', {
            email: email,
            licenseKey: licenseKey,
          });
          console.log('License validation response:', response);

          if (response.valid) {
            // 保存到 localStorage
            localStorage.setItem('email', email);
            localStorage.setItem('licenseKey', licenseKey);
            this.hasLicence = true; // 更新许可证状态
            this.$showModal('许可证验证成功！');
          } else {
            this.$showModal('许可证无效：' + response.message);
          }
        } catch (error) {
          this.$showModal('许可证验证失败：' + error);
        }
      }
    },
  },
  async mounted() {
    const savedLicenseKey = localStorage.getItem('licenseKey');
    if (savedLicenseKey) {
      this.hasLicence = true;
    }
  },
};
</script>

<style scoped>
.overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7); /* 半透明黑色背景 */
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  z-index: 10; /* 确保遮盖层在最上层 */
}

.overlay-text {
  color: white;
  font-size: 1.5rem;
  margin-bottom: 20px;
}

.overlay button {
  padding: 10px 20px;
  font-size: 1rem;
  color: white;
  background-color: #007bff;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

.overlay button:hover {
  background-color: #0056b3;
}
</style>
