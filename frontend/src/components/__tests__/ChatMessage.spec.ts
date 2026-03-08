import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import ChatMessage from '../ChatMessage.vue'
import { ChatMessage as ChatMessageType } from '../../stores/conversation'

describe('ChatMessage', () => {
  beforeEach(() => {
    vi.stubGlobal('matchMedia', (query: string) => ({
      matches: query.includes('prefers-reduced-motion'),
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn()
    }))
  })

  it('applies shimmer animation class to streaming assistant messages', () => {
    vi.stubGlobal('matchMedia', (query: string) => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn()
    }))

    const message: ChatMessageType & { isStreaming?: boolean } = {
      id: '1',
      role: 'assistant',
      content: 'Streaming content',
      timestamp: new Date(),
      isStreaming: true
    }

    const wrapper = mount(ChatMessage, {
      props: { message }
    })

    expect(wrapper.find('.message').classes()).toContain('shimmer')
  })

  it('skips animations when prefers-reduced-motion is active', () => {
    vi.stubGlobal('matchMedia', (query: string) => ({
      matches: query.includes('prefers-reduced-motion'),
      media: query,
      onchange: null,
      addListener: vi.fn(),
      removeListener: vi.fn(),
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn()
    }))

    const message: ChatMessageType & { isStreaming?: boolean } = {
      id: '1',
      role: 'assistant',
      content: 'Streaming content',
      timestamp: new Date(),
      isStreaming: true
    }

    const wrapper = mount(ChatMessage, {
      props: { message }
    })

    expect(wrapper.find('.message').classes()).not.toContain('shimmer')
  })
})
