import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useConversationStore } from '../../stores/conversation'

describe('ConversationStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes with empty conversation list', () => {
    const store = useConversationStore()

    expect(store.conversations).toEqual([])
    expect(store.activeConversationId).toBeNull()
  })

  it('addMessage appends message to active conversation', () => {
    const store = useConversationStore()

    const conversation = store.newConversation()

    const message = {
      id: '1',
      role: 'user' as const,
      content: 'Hello',
      timestamp: new Date()
    }

    store.addMessage(conversation.id, message)

    expect(store.activeConversation?.messages).toHaveLength(1)
    expect(store.activeConversation?.messages[0].content).toBe('Hello')
  })

  it('setActive updates the activeConversationId', () => {
    const store = useConversationStore()

    const conv1 = store.newConversation()
    const conv2 = store.newConversation()

    store.setActive(conv1.id)
    expect(store.activeConversationId).toBe(conv1.id)

    store.setActive(conv2.id)
    expect(store.activeConversationId).toBe(conv2.id)
  })
})
