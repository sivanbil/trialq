<template>
  <div id="particles-js"></div>
  <div class="tech-slogan">
    <div class="slogan-text">
      <span class="slogan-main">{{ $slogan.main}}</span>
      <span class="slogan-sub">{{ $slogan.sub }}</span>
    </div>
  </div>
</template>

<style scoped>
/* 固定 slogan */
.tech-slogan {
  position: fixed; /* 固定定位 */
  top: 5px; /* 固定在顶部 */
  left: 5px;
  right: 5px;
  z-index: 1000; /* 确保 slogan 在最上层 */
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
  background:#F0F0F0;
  border-radius: 10px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.5);
}

.slogan-text {
  text-align: center;
  color: white;
}

.slogan-main {
  font-size: 3rem;
  background: #000000;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: glow 2s infinite alternate;
}

.slogan-sub {
  display: block;
  font-size: 1.2rem;
  margin-top: 20px;
  color: #000000;
  animation: fadeIn 3s ease-in-out;
}

</style>
<script>
export default {
  name: 'CoolBanner',
}
</script>
