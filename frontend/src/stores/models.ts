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

  async function loadModels() {
    try {
      const response = await fetch('/api/models', {
        credentials: 'include'
      })

      if (!response.ok) {
        return
      }

      const payload = await response.json()
      models.value = payload.map((model: { model_id: string; display_name: string }) => ({
        id: model.model_id,
        name: model.display_name
      }))

      if (!models.value.some((model) => model.id === selectedModelId.value) && models.value[0]) {
        selectedModelId.value = models.value[0].id
      }
    } catch {
      // Keep local defaults when the API is unavailable.
    }
  }

  return {
    models,
    selectedModelId,
    setSelectedModel,
    loadModels
  }
})
