import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useAuthStore } from '../../stores/auth'

describe('AuthStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes as not authenticated', () => {
    const store = useAuthStore()

    expect(store.user).toBeNull()
    expect(store.isAuthenticated).toBe(false)
  })

  it('login action sets user and isAuthenticated', async () => {
    const store = useAuthStore()

    await store.login('testuser', 'password')

    expect(store.user).toEqual({ username: 'testuser' })
    expect(store.isAuthenticated).toBe(true)
  })

  it('logout action clears user state', async () => {
    const store = useAuthStore()

    await store.login('testuser', 'password')
    expect(store.isAuthenticated).toBe(true)

    store.logout()

    expect(store.user).toBeNull()
    expect(store.isAuthenticated).toBe(false)
  })
})
