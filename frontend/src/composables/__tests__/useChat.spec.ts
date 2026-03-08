import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'
import { defineComponent } from 'vue'
import { useChat } from '../../composables/useChat'

class MockWebSocket {
  url: string
  onopen: ((e: Event) => void) | null = null
  onmessage: ((e: MessageEvent) => void) | null = null
  onclose: ((e: CloseEvent) => void) | null = null
  onerror: ((e: Event) => void) | null = null
  readyState = 0

  constructor(url: string) {
    this.url = url
  }

  send(data: string) {}
  close() {
    this.readyState = 3
  }

  simulateOpen() {
    this.readyState = 1
    this.onopen?.(new Event('open'))
  }

  simulateMessage(data: object) {
    this.onmessage?.(new MessageEvent('message', { data: JSON.stringify(data) }))
  }
}

describe('useChat', () => {
  beforeEach(() => {
    vi.stubGlobal('WebSocket', MockWebSocket as any)
  })

  it('establishes WebSocket connection on mount', () => {
    class TestWebSocket extends MockWebSocket {
      static calls: string[] = []

      constructor(url: string) {
        super(url)
        TestWebSocket.calls.push(url)
      }
    }

    const wsConstructor = TestWebSocket as any
    vi.stubGlobal('WebSocket', wsConstructor)

    const TestComponent = defineComponent({
      setup() {
        return useChat('ws://localhost:8000/ws')
      },
      template: '<div></div>'
    })

    mount(TestComponent)

    expect(TestWebSocket.calls).toEqual(['ws://localhost:8000/ws'])
  })

  it('receives streaming tokens and assembles message', () => {
    let wsInstance: MockWebSocket | null = null
    class TestWebSocket extends MockWebSocket {
      constructor(url: string) {
        super(url)
        wsInstance = this
        setTimeout(() => this.simulateOpen(), 0)
      }
    }
    const wsConstructor = TestWebSocket as any
    vi.stubGlobal('WebSocket', wsConstructor)

    const TestComponent = defineComponent({
      setup() {
        return useChat('ws://localhost:8000/ws')
      },
      template: '<div></div>'
    })

    const wrapper = mount(TestComponent)
    
    wsInstance?.simulateMessage({ type: 'token', content: 'Hello ' })
    wsInstance?.simulateMessage({ type: 'token', content: 'world' })

    expect(wrapper.vm.message).toBe('Hello world')
  })

  it('sets isStreaming to true during streaming and false when done', () => {
    let wsInstance: MockWebSocket | null = null
    class TestWebSocket extends MockWebSocket {
      constructor(url: string) {
        super(url)
        wsInstance = this
        setTimeout(() => this.simulateOpen(), 0)
      }
    }
    const wsConstructor = TestWebSocket as any
    vi.stubGlobal('WebSocket', wsConstructor)

    const TestComponent = defineComponent({
      setup() {
        return useChat('ws://localhost:8000/ws')
      },
      template: '<div></div>'
    })

    const wrapper = mount(TestComponent)
    
    wsInstance?.simulateMessage({ type: 'token', content: 'text' })
    expect(wrapper.vm.isStreaming).toBe(true)

    wsInstance?.simulateMessage({ type: 'done' })
    expect(wrapper.vm.isStreaming).toBe(false)
  })
})
