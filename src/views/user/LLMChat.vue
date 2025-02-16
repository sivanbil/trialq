<template>
  <HeaderView/>
  <div class="response-printer">
    <!-- 显示内容的区域 -->
    <div class="content-box">
      <div
          v-for="(messagePair, index) in messageHistoryChunks"
          :key="index"
          :class="index % 2 === 0 ? 'bg-gray-100' : 'bg-white'"
      >
        <div v-if="messagePair.user" class="message user-message">
          <div class="avatar">Q</div>
          <div class="message-content" v-html="messagePair.user.content"></div>
        </div>
        <div v-if="messagePair.ai" class="message ai-message">
          <div class="avatar">A</div>
          <div class="message-content" v-html="messagePair.ai.content"></div>
        </div>
        <hr v-if="index < messageHistoryChunks.length - 1" class="my-4 border-gray-300">
      </div>
    </div>
    <!-- 模式切换 -->
    <div class="mode-switch flex space-x-4 mt-4 bg-white p-4 rounded-lg shadow-md">
      <label class="flex items-center cursor-pointer px-4 py-2 rounded-md hover:bg-green-100 transition-colors">
        <input type="radio" v-model="mode" value="fast" class="mr-2 text-green-500 focus:ring-green-500" />
        <span class="text-gray-700">快速回答</span>
      </label>
      <label class="flex items-center cursor-pointer px-4 py-2 rounded-md hover:bg-green-100 transition-colors">
        <input type="radio" v-model="mode" value="deep" class="mr-2 text-green-500 focus:ring-green-500" />
        <span class="text-gray-700">深度思考</span>
      </label>
    </div>

    <!-- 输入框 -->
    <div class="input-box">
      <input
          v-model="userInput"
          type="text"
          placeholder="请输入内容"
          @keyup.enter="handleInput"
      />
      <!-- 下拉选择框 -->
      <select v-model="selectedModel" class="select-box">
        <option value="deepseek-r1:1.5b">deepseek-r1:1.5b</option>
        <!-- 可以添加更多选项 -->
        <option value="deepseek-r1:7b">deepseek-r1:7b</option>
      </select>
      <button @click="handleInput" :disabled="isLoading">
        {{ isLoading ? '加载中...' : '发送' }}
      </button>
    </div>
  </div>
  <footer-view/>
</template>

<script>
import { marked } from 'marked';
import FooterView from "@/components/FooterView.vue";
import HeaderView from "@/components/HeaderView.vue";

export default {
  name: 'LLMChat',
  components: {HeaderView, FooterView},
  data() {
    return {
      userInput: '', // 用户输入的内容
      userQuestion: '', // 用户的完整问题
      messageHistory: [], // 存储历史消息
      aiAnswer: '', // AI 的完整回答
      isLoading: false, // 发送按钮的 loading 状态
      mode: 'fast', // 模式：fast（快速回答）或 deep（深度思考）
      selectedModel: 'deepseek-r1:1.5b' // 选中的模型
    };
  },
  computed: {
    messageHistoryChunks() {
      const chunks = [];
      for (let i = 0; i < this.messageHistory.length; i += 2) {
        chunks.push({
          user: this.messageHistory[i],
          ai: this.messageHistory[i + 1]
        });
      }
      return chunks;
    }
  },
  methods: {
    // 处理用户输入
    async handleInput() {
      if (this.userInput.trim() === '') return;
      this.isLoading = true; // 开始加载
      this.userQuestion = this.userInput;
      const userQuestionDisplay = marked(this.userQuestion);
      // 将用户问题添加到历史消息中
      this.messageHistory.push({ type: 'user', content: userQuestionDisplay });
      // 先添加一个空的 AI 回答占位
      this.messageHistory.push({ type: 'ai', content: '' });
      this.aiAnswer = '';
      this.userInput = '';

      try {
        const isServerAvailable = await this.checkServerAvailability();
        if (!isServerAvailable) {
          await this.$rustInvoke('execute_ollama_serve');
          // 等待一段时间让服务启动，可根据实际情况调整
          await new Promise(resolve => setTimeout(resolve, 2000));
        }

        const messages = [
          { role: 'user', content: this.userQuestion },
        ];
        await this.streamChatCompletion(messages);
      } catch (error) {
        console.error('请求出错:', error);
      } finally {
        this.isLoading = false; // 结束加载
      }
    },
    // 检测服务器是否可用
    async checkServerAvailability() {
      try {
        const response = await fetch('http://localhost:11434/api/chat', {
          method: 'HEAD',
          timeout: 2000 // 设置超时时间为 3 秒
        });
        return response.ok;
      } catch (error) {
        return false;
      }
    },
    // 流式请求
    async streamChatCompletion(messages) {
      const response = await fetch('http://localhost:11434/api/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          model: this.selectedModel, // 使用选中的模型
          messages: messages,
          temperature: this.mode === 'fast' ? 0.1 : 0.8, // 根据模式设置 temperature
          max_tokens: this.mode === 'fast' ? 50 : 500, // 根据模式设置 max_tokens
          stream: true, // 启用流式传输
        }),
      });

      const reader = response.body.getReader();
      const decoder = new TextDecoder('utf-8');
      let currentAIAnswer = '';

      // 获取最后一个 AI 消息的索引，也就是刚添加的占位消息
      const lastAIMessageIndex = this.messageHistory.length - 1;

      // eslint-disable-next-line no-constant-condition
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        // 解码流数据
        const chunk = decoder.decode(value, { stream: true });
        const lines = chunk.split('\n');

        for (const line of lines) {
          if (line.trim() === '') continue;

          try {
            const data = JSON.parse(line);
            if (data.message && data.message.content) {
              currentAIAnswer += data.message.content;
              this.aiAnswer = currentAIAnswer;
              const aiAnswerDisplay = marked(this.aiAnswer);
              // 更新最后一个 AI 消息的内容
              this.messageHistory[lastAIMessageIndex].content = aiAnswerDisplay;
            }
          } catch (error) {
            console.error('解析错误:', error);
          }
        }
      }
    },
  },
};
</script>

<style scoped>
/* 样式保持不变 */
.response-printer {
  padding: 20px;
  font-family: Arial, sans-serif;
}

.mode-switch {
  margin-bottom: 20px;
}

.mode-switch label {
  margin-right: 10px;
}

.input-box {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

input {
  flex: 1;
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 5px;
}

.select-box {
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ccc;
  border-radius: 5px;
}

button {
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 5px;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

button:hover:not(:disabled) {
  background-color: #369c6f;
}

.content-box {
  border: 1px solid #ccc;
  padding: 20px;
  height: 360px;
  overflow-y: auto;
  background-color: #f9f9f9;
}

.message {
  display: flex;
  margin-bottom: 10px;
}

.avatar {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  background-color: #42b983;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 10px;
}

.message-content {
  flex: 1;
  text-align: left;
}

.user-message {
  justify-content: flex-start;
}

.ai-message {
  justify-content: flex-start;
}
</style>
