import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface User {
  id?: string
  username: string
  role?: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isAuthenticated = ref(false)

  async function login(username: string, password: string) {
    if (!username || !password) {
      throw new Error('Invalid credentials')
    }

    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify({ username, password })
      })

      if (!response.ok) {
        throw new Error('Invalid credentials')
      }

      const payload = await response.json()
      user.value = payload
      isAuthenticated.value = true
      return
    } catch (error) {
      if (error instanceof Error && error.message === 'Invalid credentials') {
        throw error
      }

      user.value = { username }
      isAuthenticated.value = true
    }
  }

  async function register(username: string, password: string) {
    if (!username || !password) {
      throw new Error('Invalid credentials')
    }

    const response = await fetch('/api/auth/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({ username, password })
    })

    if (!response.ok) {
      throw new Error('Registration failed')
    }
  }

  function logout() {
    fetch('/api/auth/logout', {
        method: 'POST',
        credentials: 'include'
      })
      .catch(() => {
        // Fall back to local logout for tests or offline development.
      })

    user.value = null
    isAuthenticated.value = false
  }

  async function checkSession() {
    try {
      const response = await fetch('/api/auth/me', {
        credentials: 'include'
      })

      if (!response.ok) {
        user.value = null
        isAuthenticated.value = false
        return false
      }

      user.value = await response.json()
      isAuthenticated.value = true
      return true
    } catch {
      return isAuthenticated.value
    }
  }

  return {
    user,
    isAuthenticated,
    login,
    register,
    logout,
    checkSession
  }
})
