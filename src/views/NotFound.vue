<template>
  <div class="not-found">
    <h1>404 - 页面未找到</h1>
    <p>抱歉，您访问的页面不存在。</p>
    <p>3 秒后将自动跳转到首页...</p>
  </div>
</template>

<script>
export default {
  name: 'NotFound',
  mounted() {
    // 3 秒后跳转到首页
    setTimeout(() => {
      this.$router.push('/');
    }, 3000);
  },
};
</script>

<style scoped>
.not-found {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
  text-align: center;
  background-color: #f5f5f5;
  font-family: Avenir, Helvetica, Arial, sans-serif;
}

h1 {
  font-size: 2.5rem;
  color: #e81123;
}

p {
  font-size: 1.2rem;
  color: #333;
  margin-top: 10px;
}
</style>