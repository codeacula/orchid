import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import MessageList from '../MessageList.vue'
import { ChatMessage as ChatMessageType } from '../../stores/conversation'

describe('MessageList', () => {
  it('renders all messages from props', () => {
    const messages: (ChatMessageType & { isStreaming?: boolean })[] = [
      {
        id: '1',
        role: 'user',
        content: 'Hello',
        timestamp: new Date()
      },
      {
        id: '2',
        role: 'assistant',
        content: 'Hi there',
        timestamp: new Date()
      }
    ]

    const wrapper = mount(MessageList, {
      props: { messages }
    })

    expect(wrapper.findAll('[data-test="message-item"]').length).toBe(2)
  })

  it('renders empty state when no messages exist', () => {
    const wrapper = mount(MessageList, {
      props: { messages: [] }
    })

    expect(wrapper.find('[data-test="empty-state"]').exists()).toBe(true)
  })

  it('distinguishes user and assistant messages visually', () => {
    const messages: (ChatMessageType & { isStreaming?: boolean })[] = [
      {
        id: '1',
        role: 'user',
        content: 'User message',
        timestamp: new Date()
      },
      {
        id: '2',
        role: 'assistant',
        content: 'Assistant message',
        timestamp: new Date()
      }
    ]

    const wrapper = mount(MessageList, {
      props: { messages }
    })

    const items = wrapper.findAll('[data-test="message-item"]')
    expect(items[0].find('.message--user').exists()).toBe(true)
    expect(items[1].find('.message--assistant').exists()).toBe(true)
  })

  it('renders markdown content in assistant messages', () => {
    const messages: (ChatMessageType & { isStreaming?: boolean })[] = [
      {
        id: '1',
        role: 'assistant',
        content: '**bold text** and `code`',
        timestamp: new Date()
      }
    ]

    const wrapper = mount(MessageList, {
      props: { messages }
    })

    const content = wrapper.html()
    expect(content).toContain('<strong>')
    expect(content).toContain('<code>')
  })

  it('sanitizes HTML to prevent XSS', () => {
    const messages: (ChatMessageType & { isStreaming?: boolean })[] = [
      {
        id: '1',
        role: 'assistant',
        content: 'Safe content <script>alert("xss")</script>',
        timestamp: new Date()
      }
    ]

    const wrapper = mount(MessageList, {
      props: { messages }
    })

    const html = wrapper.html()
    expect(html).not.toContain('<script>')
  })

  it('message list has role="log" for accessibility', () => {
    const wrapper = mount(MessageList, {
      props: { messages: [] }
    })

    const messageList = wrapper.find('.message-list')
    expect(messageList.attributes('role')).toBe('log')
    expect(messageList.attributes('aria-live')).toBe('polite')
  })
})
