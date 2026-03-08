<template>
  <div :class="['message', messageClass]">
    <div v-if="msg.role === 'assistant'" class="content markdown">
      <MarkdownContent :content="msg.content" />
    </div>
    <div v-else class="content">
      {{ msg.content }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { ChatMessage as ChatMessageType } from '../stores/conversation'
import MarkdownContent from './MarkdownContent.vue'

defineOptions({
  name: 'ChatMessage'
})

const props = defineProps<{
  message: ChatMessageType & { isStreaming?: boolean }
}>()

const msg = props.message

const messageClass = computed(() => {
  const classes = [`message--${msg.role}`]
  
  // Check for prefers-reduced-motion
  const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches
  
  if (msg.isStreaming && msg.role === 'assistant' && !prefersReducedMotion) {
    classes.push('shimmer')
  }
  
  return classes
})
</script>

<style scoped>
.message {
  display: flex;
  padding: 0.75rem;
  border-radius: 0.25rem;
  max-width: 80%;
}

.message--user {
  background-color: var(--color-primary);
  color: white;
  align-self: flex-end;
  margin-left: auto;
}

.message--assistant {
  background-color: var(--color-background);
  color: var(--color-text);
  align-self: flex-start;
}

.content {
  word-wrap: break-word;
}

.content.markdown :deep(strong) {
  font-weight: 600;
}

.content.markdown :deep(code) {
  background-color: rgba(0, 0, 0, 0.2);
  padding: 0.125rem 0.25rem;
  border-radius: 0.125rem;
  font-family: monospace;
}

.shimmer {
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
  100% {
    opacity: 1;
  }
}
</style>
