import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Model {
  id: string
  name: string
}

export const useModelsStore = defineStore('models', () => {
  const models = ref<Model[]>([
    { id: 'gpt-4', name: 'GPT-4' },
    { id: 'gpt-3.5-turbo', name: 'GPT-3.5 Turbo' },
    { id: 'claude-3', name: 'Claude 3' }
  ])

  const selectedModelId = ref<string>('gpt-4')

  function setSelectedModel(modelId: string) {
    selectedModelId.value = modelId
  }

  return {
    models,
    selectedModelId,
    setSelectedModel
  }
})
