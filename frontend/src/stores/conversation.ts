import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: Date
  modelId?: string
  isStreaming?: boolean
}

export interface Conversation {
  id: string
  title: string
  messages: ChatMessage[]
  createdAt: Date
  updatedAt?: Date
  archived?: boolean
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
      conversation.updatedAt = message.timestamp
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
      createdAt: new Date(),
      updatedAt: new Date(),
      archived: false
    }
    conversations.value.push(conversation)
    setActive(id)
    return conversation
  }

  function upsertConversation(conversation: Conversation) {
    const existing = conversations.value.findIndex(item => item.id === conversation.id)
    if (existing >= 0) {
      conversations.value[existing] = conversation
    } else {
      conversations.value.unshift(conversation)
    }
  }

  function startStreamingAssistantMessage(conversationId: string) {
    const conversation = conversations.value.find(c => c.id === conversationId)
    if (!conversation) return

    const message: ChatMessage = {
      id: `stream-${Date.now()}`,
      role: 'assistant',
      content: '',
      timestamp: new Date(),
      isStreaming: true
    }
    conversation.messages.push(message)
  }

  function appendStreamingToken(conversationId: string, token: string) {
    const conversation = conversations.value.find(c => c.id === conversationId)
    const message = [...(conversation?.messages || [])]
      .reverse()
      .find(item => item.role === 'assistant' && item.isStreaming)
    if (!message) return
    message.content += token
  }

  function finishStreamingAssistantMessage(conversationId: string) {
    const conversation = conversations.value.find(c => c.id === conversationId)
    const message = [...(conversation?.messages || [])]
      .reverse()
      .find(item => item.role === 'assistant' && item.isStreaming)
    if (!message) return
    message.isStreaming = false
  }

  async function loadConversations() {
    try {
      const response = await fetch('/api/conversations', {
        credentials: 'include'
      })
      if (!response.ok) return
      const payload = await response.json()
      conversations.value = payload.map((item: { id: string; title: string; updated_at: string }) => ({
        id: item.id,
        title: item.title,
        messages: [],
        createdAt: new Date(item.updated_at),
        updatedAt: new Date(item.updated_at),
        archived: false
      }))
      if (!activeConversationId.value && conversations.value[0]) {
        activeConversationId.value = conversations.value[0].id
      }
    } catch {
      // Local state remains available for tests/offline work.
    }
  }

  async function loadConversation(conversationId: string) {
    try {
      const response = await fetch(`/api/conversations/${conversationId}`, {
        credentials: 'include'
      })
      if (!response.ok) return
      const payload = await response.json()
      const history = payload.history
      upsertConversation({
        id: history.id,
        title: history.title,
        messages: history.messages.map((message: any) => ({
          id: `${history.id}-${message.timestamp}-${message.role}`,
          role: message.role,
          content: message.content,
          timestamp: new Date(message.timestamp),
          modelId: message.model_id
        })),
        createdAt: new Date(history.updated_at),
        updatedAt: new Date(history.updated_at),
        archived: history.archived
      })
    } catch {
      // Ignore when the backend is unavailable.
    }
  }

  async function createConversation(title?: string, draftId?: string) {
    try {
      const response = await fetch('/api/conversations', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify({ title })
      })

      if (response.ok) {
        const payload = await response.json()
        const conversation: Conversation = {
          id: payload.id,
          title: title || `Chat ${new Date().toLocaleString()}`,
          messages: [],
          createdAt: new Date(),
          updatedAt: new Date(),
          archived: false
        }

        if (draftId) {
          conversations.value = conversations.value.filter(item => item.id !== draftId)
        }
        upsertConversation(conversation)
        setActive(conversation.id)
        return conversation
      }
    } catch {
      // Fall through to local conversation creation.
    }

    return newConversation()
  }

  return {
    conversations,
    activeConversationId,
    activeConversation,
    addMessage,
    setActive,
    newConversation,
    upsertConversation,
    startStreamingAssistantMessage,
    appendStreamingToken,
    finishStreamingAssistantMessage,
    loadConversations,
    loadConversation,
    createConversation
  }
})
