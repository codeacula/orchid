import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import Sidebar from '../Sidebar.vue'
import { useConversationStore } from '../../stores/conversation'

describe('Sidebar', () => {
  it('renders all conversations from store', () => {
    const wrapper = mount(Sidebar, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            conversation: {
              conversations: [
                { id: '1', title: 'Conv 1', messages: [], createdAt: new Date() },
                { id: '2', title: 'Conv 2', messages: [], createdAt: new Date() }
              ],
              activeConversationId: null
            }
          }
        })]
      }
    })

    expect(wrapper.findAll('[data-test="conversation-item"]').length).toBe(2)
  })

  it('highlights the active conversation', () => {
    const wrapper = mount(Sidebar, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            conversation: {
              conversations: [
                { id: '1', title: 'Conv 1', messages: [], createdAt: new Date() },
                { id: '2', title: 'Conv 2', messages: [], createdAt: new Date() }
              ],
              activeConversationId: '1'
            }
          }
        })]
      }
    })

    const items = wrapper.findAll('[data-test="conversation-item"]')
    expect(items[0].classes()).toContain('conversation--active')
  })

  it('clicking a conversation calls setActive on store', async () => {
    const wrapper = mount(Sidebar, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            conversation: {
              conversations: [
                { id: '1', title: 'Conv 1', messages: [], createdAt: new Date() },
                { id: '2', title: 'Conv 2', messages: [], createdAt: new Date() }
              ],
              activeConversationId: null
            }
          }
        })]
      }
    })

    const store = useConversationStore()
    const items = wrapper.findAll('[data-test="conversation-item"]')
    await items[0].trigger('click')

    expect(store.setActive).toHaveBeenCalledWith('1')
  })

  it('new chat button calls newConversation on store', async () => {
    const wrapper = mount(Sidebar, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            conversation: {
              conversations: [],
              activeConversationId: null
            }
          }
        })]
      }
    })

    const store = useConversationStore()
    const button = wrapper.find('.new-chat-btn')
    await button.trigger('click')

    expect(store.newConversation).toHaveBeenCalled()
  })
})
