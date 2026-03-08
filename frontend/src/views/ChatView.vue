<template>
  <div class="chat-view">
    <div class="chat-toolbar">
      <ModelSelector @model-change="handleModelChange" />
    </div>
    <MessageList :messages="messages" />
    <StreamingIndicator :streaming="isStreaming" :partial-content="streamingMessage" />
    <MessageInput :disabled="isStreaming || !activeConversationId" @send="handleSendMessage" />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useConversationStore } from '../stores/conversation'
import { useModelsStore } from '../stores/models'
import { useChat } from '../composables/useChat'
import MessageList from '../components/MessageList.vue'
import MessageInput from '../components/MessageInput.vue'
import ModelSelector from '../components/ModelSelector.vue'
import StreamingIndicator from '../components/StreamingIndicator.vue'

const conversationStore = useConversationStore()
const modelsStore = useModelsStore()

const { isStreaming, message: streamingMessage, sendChatMessage } = useChat(undefined, {
  onToken(conversationId, content) {
    conversationStore.appendStreamingToken(conversationId, content)
  },
  onDone(conversationId) {
    conversationStore.finishStreamingAssistantMessage(conversationId)
    void conversationStore.loadConversation(conversationId)
    void conversationStore.loadConversations()
  },
  onError(conversationId) {
    conversationStore.finishStreamingAssistantMessage(conversationId)
  }
})

const messages = computed(() => {
  return conversationStore.activeConversation?.messages || []
})

const activeConversationId = computed(() => conversationStore.activeConversationId)

onMounted(async () => {
  await modelsStore.loadModels()
  await conversationStore.loadConversations()

  if (conversationStore.activeConversationId) {
    await conversationStore.loadConversation(conversationStore.activeConversationId)
  }
})

async function handleSendMessage(content: string) {
  let conversationId = conversationStore.activeConversationId
  if (!conversationId) {
    const conversation = await conversationStore.createConversation()
    conversationId = conversation.id
  }

  if (conversationId && content.trim()) {
    conversationStore.addMessage(conversationId, {
      id: Date.now().toString(),
      role: 'user',
      content,
      timestamp: new Date(),
      modelId: modelsStore.selectedModelId
    })
    conversationStore.startStreamingAssistantMessage(conversationId)
    sendChatMessage({
      conversationId,
      content,
      modelId: modelsStore.selectedModelId
    })
  }
}

function handleModelChange(modelId: string) {
  modelsStore.setSelectedModel(modelId)
}
</script>

<style scoped>
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.chat-toolbar {
  border-bottom: 1px solid var(--color-border);
}
</style>
