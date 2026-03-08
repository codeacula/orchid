import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import ModelSelector from '../ModelSelector.vue'
import { useModelsStore } from '../../stores/models'

describe('ModelSelector', () => {
  it('renders available models as dropdown options', () => {
    const wrapper = mount(ModelSelector, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            models: {
              models: [
                { id: 'gpt-4', name: 'GPT-4' },
                { id: 'gpt-3.5', name: 'GPT-3.5' }
              ],
              selectedModelId: 'gpt-4'
            }
          }
        })]
      }
    })

    const options = wrapper.findAll('option')
    expect(options.length).toBe(2)
  })

  it('emits model-change event when selection changes', async () => {
    const wrapper = mount(ModelSelector, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            models: {
              models: [
                { id: 'gpt-4', name: 'GPT-4' },
                { id: 'gpt-3.5', name: 'GPT-3.5' }
              ],
              selectedModelId: 'gpt-4'
            }
          }
        })]
      }
    })

    const select = wrapper.find('.model-select')
    await select.setValue('gpt-3.5')

    expect(wrapper.emitted('model-change')).toBeTruthy()
    expect(wrapper.emitted('model-change')?.[0]).toEqual(['gpt-3.5'])
  })

  it('displays currently selected model', () => {
    const wrapper = mount(ModelSelector, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn,
          initialState: {
            models: {
              models: [
                { id: 'gpt-4', name: 'GPT-4' },
                { id: 'gpt-3.5', name: 'GPT-3.5' }
              ],
              selectedModelId: 'gpt-4'
            }
          }
        })]
      }
    })

    const select = wrapper.find('.model-select') as any
    expect(select.element.value).toBe('gpt-4')
  })
})
