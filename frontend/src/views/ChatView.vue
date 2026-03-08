<template>
  <div class="chat-view">
    <div class="chat-toolbar">
      <ModelSelector @model-change="handleModelChange" />
    </div>
    <div v-if="showWelcome" class="welcome-state">
      <p class="welcome-kicker">Orchid is ready</p>
      <h2>Start your first conversation</h2>
      <p>Pick a model and send a message to begin. A fresh chat is ready for you.</p>
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
const showWelcome = computed(() => Boolean(activeConversationId.value && messages.value.length === 0))

onMounted(async () => {
  await modelsStore.loadModels()
  await conversationStore.loadConversations()

  if (!conversationStore.activeConversationId) {
    await conversationStore.createConversation()
  }

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

.welcome-state {
  padding: 2rem 1rem 0;
}

.welcome-kicker {
  color: var(--color-primary);
  font-size: 0.85rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.welcome-state h2 {
  margin-top: 0.5rem;
  font-size: 1.8rem;
}

.welcome-state p:last-child {
  margin-top: 0.5rem;
  color: rgba(243, 244, 246, 0.8);
}
</style>
