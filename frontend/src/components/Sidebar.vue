<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <h1>Orchid Chat</h1>
      <button @click="handleNewChat" class="new-chat-btn">
        + New Chat
      </button>
    </div>
    <div class="conversations-list">
      <div
        v-for="conv in store.conversations"
        :key="conv.id"
        data-test="conversation-item"
        :class="['conversation-item', {
          'conversation--active': store.activeConversationId === conv.id
        }]"
        @click="handleSelectConversation(conv.id)"
      >
        {{ conv.title }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useConversationStore } from '../stores/conversation'

defineOptions({
  name: 'Sidebar'
})

const store = useConversationStore()

onMounted(async () => {
  await store.loadConversations()
})

function handleSelectConversation(conversationId: string) {
  store.setActive(conversationId)
}

function handleNewChat() {
  const draft = store.newConversation()
  if (draft) {
    void store.createConversation(draft.title, draft.id)
  }
}
</script>

<style scoped>
.sidebar {
  width: 250px;
  background-color: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  height: 100vh;
  padding: 0;
}

.sidebar-header {
  padding: 1rem;
  border-bottom: 1px solid var(--color-border);
}

.sidebar-header h1 {
  margin: 0 0 1rem 0;
  font-size: 1.5rem;
  color: var(--color-text);
}

.new-chat-btn {
  width: 100%;
  padding: 0.75rem;
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  font-weight: 600;
}

.new-chat-btn:hover {
  opacity: 0.9;
}

.conversations-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.conversation-item {
  padding: 0.75rem;
  margin: 0.25rem 0;
  border-radius: 0.25rem;
  cursor: pointer;
  color: var(--color-text);
  transition: background-color 0.2s;
}

.conversation-item:hover {
  background-color: var(--color-border);
}

.conversation--active {
  background-color: var(--color-primary);
  color: white;
  font-weight: 600;
}
</style>
