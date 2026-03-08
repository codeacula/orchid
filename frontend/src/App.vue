<template>
  <div class="app dark-theme">
    <Sidebar />
    <div class="main-content">
      <RouterView />
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import Sidebar from './components/Sidebar.vue'
</script>

<style>
:root {
  --color-primary: #8B5CF6;
  --color-background: #1f2937;
  --color-surface: #111827;
  --color-border: #374151;
  --color-text: #f3f4f6;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
  background-color: var(--color-background);
  color: var(--color-text);
}

#app {
  height: 100vh;
}
</style>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  background-color: var(--color-background);
}

.app.dark-theme {
  color-scheme: dark;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
