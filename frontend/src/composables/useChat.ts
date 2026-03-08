import { ref, onUnmounted } from 'vue'

interface ChatCallbacks {
  onToken?: (conversationId: string, content: string) => void
  onDone?: (conversationId: string) => void
  onError?: (conversationId: string, message: string) => void
}

function defaultWsUrl() {
  if (typeof window === 'undefined' || !window.location) {
    return 'ws://localhost:3000/ws'
  }

  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
  return `${protocol}//${window.location.host}/ws`
}

export function useChat(wsUrl: string = defaultWsUrl(), callbacks: ChatCallbacks = {}) {
  const message = ref('')
  const isStreaming = ref(false)
  const connected = ref(false)
  const lastError = ref('')
  
  const ws: WebSocket = (globalThis as any).WebSocket(wsUrl)

  ws.onopen = () => {
    connected.value = true
  }

  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data)
      const conversationId = data.conversation_id || ''
      
      if (data.type === 'token') {
        isStreaming.value = true
        message.value += data.content
        callbacks.onToken?.(conversationId, data.content)
      } else if (data.type === 'done') {
        isStreaming.value = false
        callbacks.onDone?.(conversationId)
      } else if (data.type === 'error') {
        isStreaming.value = false
        lastError.value = data.message || 'Streaming failed'
        callbacks.onError?.(conversationId, lastError.value)
      }
    } catch (e) {
      console.error('Failed to parse message:', e)
    }
  }

  ws.onerror = (error) => {
    console.error('WebSocket error:', error)
    isStreaming.value = false
    connected.value = false
    lastError.value = 'WebSocket error'
  }

  ws.onclose = () => {
    isStreaming.value = false
    connected.value = false
  }

  onUnmounted(() => {
    ws.close()
  })

  function sendMessage(content: string) {
    if (ws && ws.readyState === WebSocket.OPEN) {
      if (typeof content === 'string') {
        ws.send(JSON.stringify({
          type: 'send_message',
          content
        }))
        return
      }
    }
  }

  function sendChatMessage(payload: { conversationId: string; content: string; modelId?: string }) {
    if (ws && ws.readyState === WebSocket.OPEN) {
      message.value = ''
      lastError.value = ''
      ws.send(JSON.stringify({
        type: 'send_message',
        conversation_id: payload.conversationId,
        content: payload.content,
        model_id: payload.modelId
      }))
    }
  }

  function close() {
    ws.close()
  }

  return {
    message,
    isStreaming,
    connected,
    lastError,
    sendMessage,
    sendChatMessage,
    close
  }
}
