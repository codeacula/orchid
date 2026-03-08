<template>
  <div class="chat-view">
    <MessageList :messages="messages" />
    <MessageInput @send="handleSendMessage" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useConversationStore } from '../stores/conversation'
import MessageList from '../components/MessageList.vue'
import MessageInput from '../components/MessageInput.vue'

const conversationStore = useConversationStore()

const messages = computed(() => {
  return conversationStore.activeConversation?.messages || []
})

function handleSendMessage(content: string) {
  if (conversationStore.activeConversationId && content.trim()) {
    const message = {
      id: Date.now().toString(),
      role: 'user' as const,
      content,
      timestamp: new Date()
    }
    conversationStore.addMessage(conversationStore.activeConversationId, message)
  }
}
</script>

<style scoped>
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
}
</style>
