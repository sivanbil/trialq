<template>
  <div class="reminder-modal">
    <div class="modal-content">
      <p>{{ message }}</p>
      <div class="button-group">
        <button @click="close">关闭</button>
        <button @click="dismiss">不再提醒</button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ReminderModal',
  props: {
    message: {
      type: String,
      required: true,
    },
  },
  methods: {
    close() {
      this.$emit('close');
    },
    dismiss() {
      this.$emit('dismiss');
    },
  },
};
</script>

<style scoped>
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

.button-group {
  display: flex;
  justify-content: center;
  gap: 10px;
}

.button-group button {
  padding: 10px 20px;
  background-color: #4b0082;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.button-group button:hover {
  background-color: #3a0069;
}
</style>