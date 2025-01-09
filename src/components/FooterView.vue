<template>
  <footer class="bg-gray-800 text-white py-6" style="position: fixed;width: 100%;bottom: 0;">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- 底部版权信息 -->
      <div class="border-gray-700 pt-6 text-center">
        <p class="text-sm text-gray-400">
          &copy; {{ $slogan.year }} {{ $slogan.main }}. 保留所有权利. | 联系我们：<a href="mailto:{{ $slogan.email }}}" class="text-sm text-gray-400 hover:text-white transition-colors">
          {{ $slogan.email }}
        </a>
        </p>
      </div>
    </div>
  </footer>
</template>

<script>
export default {
  name: 'FooterView',
};
</script>

<style scoped>
/* 如果需要自定义样式，可以在这里添加 */
</style>