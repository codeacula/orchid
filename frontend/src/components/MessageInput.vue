<template>
  <form @submit.prevent="handleSubmit" class="message-input">
    <input
      v-model="input"
      type="text"
      placeholder="Type a message..."
      :disabled="disabled"
      class="input-field"
    />
    <button :disabled="disabled" class="send-button" type="submit">
      Send
    </button>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineOptions({
  name: 'MessageInput'
})

defineProps<{
  disabled?: boolean
}>()

const emit = defineEmits<{
  send: [content: string]
}>()

const input = ref('')

function handleSubmit() {
  const trimmed = input.value.trim()
  if (trimmed) {
    emit('send', trimmed)
    input.value = ''
  }
}
</script>

<style scoped>
.message-input {
  display: flex;
  gap: 0.5rem;
  padding: 1rem;
  border-top: 1px solid var(--color-border);
  background-color: var(--color-surface);
}

.input-field {
  flex: 1;
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  border-radius: 0.25rem;
  background-color: var(--color-background);
  color: var(--color-text);
}

.input-field:focus {
  outline: none;
  border-color: var(--color-primary);
}

.send-button {
  padding: 0.75rem 1.5rem;
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  font-weight: 600;
}

.send-button:hover:not(:disabled) {
  opacity: 0.9;
}

.send-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
