import { describe, it, expect, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createTestingPinia } from '@pinia/testing'
import LoginForm from '../LoginForm.vue'
import { useAuthStore } from '../../stores/auth'

const push = vi.fn()

vi.mock('vue-router', () => ({
  useRouter: () => ({
    push
  })
}))

describe('LoginForm', () => {
  it('submits credentials on form submit', async () => {
    push.mockReset()
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
    expect(push).toHaveBeenCalledWith('/')
  })

  it('displays error message on login failure', async () => {
    push.mockReset()
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
    expect(push).not.toHaveBeenCalled()
  })

  it('disables submit button during login attempt', async () => {
    push.mockReset()
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
