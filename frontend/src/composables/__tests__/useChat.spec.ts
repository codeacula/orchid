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
    const wsConstructor = vi.fn((url: string) => {
      return new MockWebSocket(url)
    })
    vi.stubGlobal('WebSocket', wsConstructor)

    const TestComponent = defineComponent({
      setup() {
        return useChat('ws://localhost:8000/ws')
      },
      template: '<div></div>'
    })

    mount(TestComponent)

    expect(wsConstructor).toHaveBeenCalledWith('ws://localhost:8000/ws')
  })

  it('receives streaming tokens and assembles message', () => {
    const wsConstructor = vi.fn((url: string) => {
      const ws = new MockWebSocket(url)
      setTimeout(() => ws.simulateOpen(), 0)
      return ws
    })
    vi.stubGlobal('WebSocket', wsConstructor)

    let wsInstance: MockWebSocket | null = null
    const TestComponent = defineComponent({
      setup() {
        const result = useChat('ws://localhost:8000/ws')
        wsInstance = (wsConstructor as any).mock.results[0].value
        return result
      },
      template: '<div></div>'
    })

    const wrapper = mount(TestComponent)
    
    wsInstance?.simulateMessage({ type: 'token', content: 'Hello ' })
    wsInstance?.simulateMessage({ type: 'token', content: 'world' })

    expect(wrapper.vm.message).toBe('Hello world')
  })

  it('sets isStreaming to true during streaming and false when done', () => {
    const wsConstructor = vi.fn((url: string) => {
      const ws = new MockWebSocket(url)
      setTimeout(() => ws.simulateOpen(), 0)
      return ws
    })
    vi.stubGlobal('WebSocket', wsConstructor)

    let wsInstance: MockWebSocket | null = null
    const TestComponent = defineComponent({
      setup() {
        const result = useChat('ws://localhost:8000/ws')
        wsInstance = (wsConstructor as any).mock.results[0].value
        return result
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
