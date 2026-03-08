import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import MessageInput from '../MessageInput.vue'

describe('MessageInput', () => {
  it('emits send event with input content on submit', async () => {
    const wrapper = mount(MessageInput)

    const input = wrapper.find('.input-field')
    await input.setValue('Hello world')

    await wrapper.find('form').trigger('submit')

    expect(wrapper.emitted('send')).toBeTruthy()
    expect(wrapper.emitted('send')?.[0]).toEqual(['Hello world'])
  })

  it('clears input after submission', async () => {
    const wrapper = mount(MessageInput)

    const input = wrapper.find('.input-field') as any
    await input.setValue('Test message')
    await wrapper.find('form').trigger('submit')

    expect(input.element.value).toBe('')
  })

  it('does not submit empty or whitespace-only message', async () => {
    const wrapper = mount(MessageInput)

    const input = wrapper.find('.input-field')
    await input.setValue('   ')

    await wrapper.find('form').trigger('submit')

    expect(wrapper.emitted('send')).toBeFalsy()
  })

  it('disables send button while streaming', async () => {
    const wrapper = mount(MessageInput, {
      props: { disabled: true }
    })

    const button = wrapper.find('.send-button')
    expect(button.attributes('disabled')).toBe('')
  })
})
