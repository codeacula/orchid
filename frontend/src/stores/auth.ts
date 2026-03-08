import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface User {
  username: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isAuthenticated = ref(false)

  async function login(username: string, password: string) {
    // Simulate login - in real app would call API
    if (!username || !password) {
      throw new Error('Invalid credentials')
    }
    user.value = { username }
    isAuthenticated.value = true
  }

  function logout() {
    user.value = null
    isAuthenticated.value = false
  }

  return {
    user,
    isAuthenticated,
    login,
    logout
  }
})
