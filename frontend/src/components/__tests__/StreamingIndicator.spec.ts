import { describe, it, expect } from 'vitest'
import { mount } from '@vue/test-utils'
import StreamingIndicator from '../StreamingIndicator.vue'

describe('StreamingIndicator', () => {
  it('is visible when streaming is active', () => {
    const wrapper = mount(StreamingIndicator, {
      props: {
        streaming: true,
        partialContent: 'Hello...'
      }
    })

    expect(wrapper.find('.streaming-indicator').exists()).toBe(true)
    expect(wrapper.find('.streaming-indicator').attributes('aria-label')).toBeTruthy()
  })

  it('displays partial content as it arrives', async () => {
    const wrapper = mount(StreamingIndicator, {
      props: {
        streaming: true,
        partialContent: 'Hello'
      }
    })

    expect(wrapper.text()).toContain('Hello')

    await wrapper.setProps({ partialContent: 'Hello world' })
    expect(wrapper.text()).toContain('Hello world')
  })

  it("has aria-live='polite' for screen reader announcements", () => {
    const wrapper = mount(StreamingIndicator, {
      props: {
        streaming: true,
        partialContent: 'Content'
      }
    })

    const indicator = wrapper.find('.streaming-indicator')
    expect(indicator.attributes('aria-live')).toBe('polite')
  })
})
