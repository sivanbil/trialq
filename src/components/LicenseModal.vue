<template>
  <div v-if="visible" class="modal">
    <div class="modal-content">
      <span class="close" @click="closeModal">&times;</span>
      <h2>输入注册码</h2>

      <!-- 企业邮箱输入 -->
      <input
          v-model="email"
          placeholder="请输入申请许可证密钥的企业邮箱"
          :disabled="true"
          :readonly="true"
          :class="{ 'input-error': emailError }"
      />
      <p v-if="emailError" class="error-message">企业邮箱为必填项</p>

      <!-- 许可证密钥输入 -->
      <textarea
          v-model="licenseKey"
          placeholder="请输入许可证密钥"
          rows="5"
          :class="{ 'input-error': licenseKeyError }"
      ></textarea>
      <p v-if="licenseKeyError" class="error-message">许可证密钥为必填项</p>

      <!-- 保存按钮 -->
      <button @click="saveLicense">保存</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'LicenseModal',
  props: {
    visible: {
      type: Boolean,
      required: true,
    },
  },
  data() {
    return {
      email: 'sivanliaobil@gmail.com', // 企业邮箱
      licenseKey: '', // 许可证密钥
      emailError: false, // 邮箱输入错误状态
      licenseKeyError: false, // 许可证输入错误状态
    };
  },
  methods: {
    closeModal() {
      this.$emit('close'); // 通知父组件关闭弹窗
    },
    saveLicense() {
      // 重置错误状态
      this.emailError = false;
      this.licenseKeyError = false;

      // 校验企业邮箱
      if (!this.email) {
        this.emailError = true;
      }

      // 校验许可证密钥
      if (!this.licenseKey) {
        this.licenseKeyError = true;
      }

      // 如果校验未通过，显示错误提示并停止保存
      if (this.emailError || this.licenseKeyError) {
        alert('请填写完整的企业邮箱和许可证密钥');
        return;
      }

      // 校验通过，通知父组件保存许可证
      this.$emit('save', {
        email: this.email,
        licenseKey: this.licenseKey,
      });

      // 清空输入框
      this.email = '';
      this.licenseKey = '';

      // 关闭弹窗
      this.closeModal();
    },
  },
};
</script>

<style scoped>
/* 弹窗样式 */
.modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal-content {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  width: 400px;
  text-align: center;
}

.close {
  float: right;
  font-size: 1.5rem;
  cursor: pointer;
}

input,
textarea {
  width: 90%;
  padding: 10px;
  margin: 10px 0;
  border: 1px solid #ccc;
  border-radius: 4px;
  resize: vertical; /* 允许垂直调整大小 */
  font-family: inherit;
  font-size: 1rem;
}

.input-error {
  border-color: #e81123; /* 输入框错误状态时的边框颜色 */
}

.error-message {
  color: #e81123; /* 错误提示文字颜色 */
  font-size: 0.875rem;
  margin: 0 0 10px 0;
  text-align: left;
  padding-left: 10px;
}

button {
  padding: 10px 20px;
  background-color: #4b0082;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background-color: #3a0069;
}
</style>