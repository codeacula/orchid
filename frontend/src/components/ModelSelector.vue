<template>
  <div class="model-selector">
    <label for="model-select">Model:</label>
    <select
      id="model-select"
      :value="store.selectedModelId"
      @change="handleModelChange"
      class="model-select"
    >
      <option
        v-for="model in store.models"
        :key="model.id"
        :value="model.id"
      >
        {{ model.name }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { useModelsStore } from '../stores/models'

defineOptions({
  name: 'ModelSelector'
})

const emit = defineEmits<{
  'model-change': [modelId: string]
}>()

const store = useModelsStore()

function handleModelChange(event: Event) {
  const target = event.target as HTMLSelectElement
  emit('model-change', target.value)
  store.setSelectedModel(target.value)
}
</script>

<style scoped>
.model-selector {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  background-color: var(--color-surface);
}

label {
  color: var(--color-text);
  font-weight: 600;
}

.model-select {
  flex: 1;
  padding: 0.5rem;
  background-color: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: 0.25rem;
}

.model-select:focus {
  outline: none;
  border-color: var(--color-primary);
}
</style>
