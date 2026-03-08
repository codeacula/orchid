import { describe, it, expect, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import LoginForm from '../LoginForm.vue'
import { useAuthStore } from '../../stores/auth'

describe('LoginForm', () => {
  it('submits credentials on form submit', async () => {
    const wrapper = mount(LoginForm, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn
        })]
      }
    })

    const store = useAuthStore()
    vi.mocked(store.login).mockResolvedValue(undefined)

    const usernameInput = wrapper.find('#username')
    const passwordInput = wrapper.find('#password')

    await usernameInput.setValue('testuser')
    await passwordInput.setValue('password123')

    await wrapper.find('form').trigger('submit')
    await flushPromises()

    expect(store.login).toHaveBeenCalledWith('testuser', 'password123')
  })

  it('displays error message on login failure', async () => {
    const wrapper = mount(LoginForm, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn
        })]
      }
    })

    const store = useAuthStore()
    vi.mocked(store.login).mockRejectedValue(new Error('Invalid credentials'))

    const usernameInput = wrapper.find('#username')
    const passwordInput = wrapper.find('#password')

    await usernameInput.setValue('testuser')
    await passwordInput.setValue('wrong')

    await wrapper.find('form').trigger('submit')
    await flushPromises()

    expect(wrapper.find('.error-message').exists()).toBe(true)
    expect(wrapper.text()).toContain('Invalid credentials')
  })

  it('disables submit button during login attempt', async () => {
    const wrapper = mount(LoginForm, {
      global: {
        plugins: [createTestingPinia({
          createSpy: vi.fn
        })]
      }
    })

    const store = useAuthStore()
    let resolveLogin: any
    vi.mocked(store.login).mockImplementation(
      () => new Promise((resolve) => {
        resolveLogin = resolve
      })
    )

    const usernameInput = wrapper.find('#username')
    const passwordInput = wrapper.find('#password')

    await usernameInput.setValue('testuser')
    await passwordInput.setValue('password123')

    const submitBtn = wrapper.find('.submit-btn')
    expect(submitBtn.attributes('disabled')).toBeUndefined()

    await wrapper.find('form').trigger('submit')

    expect(submitBtn.attributes('disabled')).toBe('')

    resolveLogin()
    await flushPromises()

    expect(submitBtn.attributes('disabled')).toBeUndefined()
  })
})
