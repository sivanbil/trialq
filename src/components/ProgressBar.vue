<template>
  <div v-if="progress !== null" class="w-full relative">
    <progress
        class="w-full bg-gray-200 rounded-full h-3"
        :value="progress"
        max="100"
    ></progress>
    <span class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-xs text-gray-700">
        {{ progress }}%
      </span>
  </div>
</template>

<script>
export default {
  name: 'ProgressBar',
  props: {
    // 定义一个名为 progress 的 prop，用于接收外部传入的进度值
    progress: {
      type: Number,
      default: null
    }
  }
}
</script>

<style scoped>

</style>
