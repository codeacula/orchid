<template>
  <div v-if="streaming" class="streaming-indicator" :aria-label="`Streaming: ${partialContent}`" aria-live="polite">
    <div class="spinner"></div>
    <span>{{ partialContent }}</span>
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'StreamingIndicator'
})

defineProps<{
  streaming: boolean
  partialContent: string
}>()
</script>

<style scoped>
.streaming-indicator {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background-color: var(--color-surface);
  border-radius: 0.25rem;
  color: var(--color-text);
}

.spinner {
  display: inline-block;
  width: 1rem;
  height: 1rem;
  border: 2px solid var(--color-border);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
