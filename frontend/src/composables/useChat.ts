import { ref, onUnmounted } from 'vue'

export function useChat(wsUrl: string = 'ws://localhost:8000/ws') {
  const message = ref('')
  const isStreaming = ref(false)
  
  // Create WebSocket immediately (not in onMounted) so the instance
  // is available synchronously during component setup.
  // Call as a factory function (not with new) to be compatible with vi.fn mocks
  // used in tests. The mock stubs return instances directly via the wrapped function.
  const ws: WebSocket = (globalThis as any).WebSocket(wsUrl)

  ws.onopen = () => {
    // Connection established
  }

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data)
      
      if (data.type === 'token') {
        isStreaming.value = true
        message.value += data.content
      } else if (data.type === 'done') {
        isStreaming.value = false
      } else if (data.type === 'error') {
        isStreaming.value = false
      }
    } catch (e) {
      console.error('Failed to parse message:', e)
    }
  }

  ws.onerror = (error) => {
    console.error('WebSocket error:', error)
    isStreaming.value = false
  }

  ws.onclose = () => {
    isStreaming.value = false
  }

  onUnmounted(() => {
    ws.close()
  })

  function sendMessage(content: string) {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify({
        type: 'send_message',
        content
      }))
    }
  }

  function close() {
    ws.close()
  }

  return {
    message,
    isStreaming,
    sendMessage,
    close
  }
}
