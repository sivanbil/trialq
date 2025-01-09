<template>
  <button @click="handleBack" class="back-button">
    <span>返回</span>
  </button>
</template>

<script>
export default {
  name: 'BackButton',
  props: {
    to: {
      type: String,
      default: '', // 默认值为空，表示返回上一页
    },
  },
  methods: {
    handleBack() {
      if (this.to) {
        // 如果有配置路由，跳转到指定路由
        this.$router.push(this.to);
      } else {
        // 如果没有配置路由，返回上一页
        if (window.history.length > 1) {
          this.$router.go(-1); // 返回上一页
        } else {
          this.$router.push('/'); // 如果历史记录为空，则返回首页
        }
      }
    },
  },
};
</script>

<style scoped>
.back-button {
  padding: 10px 20px;
  background-color: #0078d7;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: background-color 0.3s ease;
}

.back-button:hover {
  background-color: #005bb5;
}
</style>