<template>
  <div class="message-list" role="log" aria-live="polite">
    <div v-if="messages.length === 0" data-test="empty-state" class="empty-state">
      <p>No messages yet. Start a conversation!</p>
    </div>
    <div v-for="msg in messages" :key="msg.id" data-test="message-item" class="message-item">
      <ChatMessage :message="msg" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ChatMessage as ChatMessageType } from '../stores/conversation'
import ChatMessage from './ChatMessage.vue'

defineOptions({
  name: 'MessageList'
})

defineProps<{
  messages: ChatMessageType[]
}>()
</script>

<style scoped>
.message-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  flex: 1;
  overflow-y: auto;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--color-text);
}

.message-item {
  display: flex;
}
</style>
