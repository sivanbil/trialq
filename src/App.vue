<template>

  <router-view />

</template>

<script>
export default {
  name: 'App',
  data() {
    return {
      showReminderModal: false, // 是否显示提醒弹窗
      reminderMessage: '', // 提醒消息内容
      licenceExpireTime: null, // 许可证过期时间（Unix 时间戳）
    };
  },
  mounted() {
    // 从 localStorage 中获取许可证过期时间
    const savedExpireTime = localStorage.getItem('licenceExpireTime');
    if (savedExpireTime) {
      this.licenceExpireTime = parseInt(savedExpireTime, 10);
    }

    // 启动定时器，每分钟检查一次
    this.checkLicenceExpiry();
    setInterval(this.checkLicenceExpiry, 60 * 1000); // 每分钟检查一次
  },
  methods: {
    // 检查许可证是否即将过期
    checkLicenceExpiry() {
      if (!this.licenceExpireTime) return;

    },
  },
};
</script>

<style>
/* 全局样式 */
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
}

/* 提醒弹窗样式 */
.reminder-modal {
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
  text-align: center;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.modal-content p {
  margin: 0 0 20px 0;
  font-size: 1.2rem;
}

.modal-content button {
  padding: 10px 20px;
  background-color: #4b0082;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.modal-content button:hover {
  background-color: #3a0069;
}
</style>
