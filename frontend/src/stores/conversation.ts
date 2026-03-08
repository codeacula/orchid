import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: Date
}

export interface Conversation {
  id: string
  title: string
  messages: ChatMessage[]
  createdAt: Date
}

export const useConversationStore = defineStore('conversation', () => {
  const conversations = ref<Conversation[]>([])
  const activeConversationId = ref<string | null>(null)

  const activeConversation = computed(() => {
    if (!activeConversationId.value) return null
    return conversations.value.find(c => c.id === activeConversationId.value)
  })

  function addMessage(conversationId: string, message: ChatMessage) {
    const conversation = conversations.value.find(c => c.id === conversationId)
    if (conversation) {
      conversation.messages.push(message)
    }
  }

  function setActive(conversationId: string) {
    activeConversationId.value = conversationId
  }

  function newConversation() {
    const id = Date.now().toString()
    const conversation: Conversation = {
      id,
      title: `Conversation ${conversations.value.length + 1}`,
      messages: [],
      createdAt: new Date()
    }
    conversations.value.push(conversation)
    setActive(id)
    return conversation
  }

  return {
    conversations,
    activeConversationId,
    activeConversation,
    addMessage,
    setActive,
    newConversation
  }
})
