<template>
  <form @submit.prevent="handleSubmit" class="login-form">
    <h2>{{ mode === 'login' ? 'Login' : 'Create account' }}</h2>
    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>
    <div class="form-group">
      <label for="username">Username:</label>
      <input
        id="username"
        v-model="username"
        type="text"
        placeholder="Enter username"
      />
    </div>
    <div class="form-group">
      <label for="password">Password:</label>
      <input
        id="password"
        v-model="password"
        type="password"
        placeholder="Enter password"
      />
    </div>
    <button :disabled="isLoading" type="submit" class="submit-btn">
      {{ isLoading ? (mode === 'login' ? 'Logging in...' : 'Creating account...') : (mode === 'login' ? 'Login' : 'Create account') }}
    </button>
    <button type="button" class="secondary-btn" :disabled="isLoading" @click="toggleMode">
      {{ mode === 'login' ? 'Need an account?' : 'Already have an account?' }}
    </button>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

defineOptions({
  name: 'LoginForm'
})

const authStore = useAuthStore()
const router = useRouter()

const username = ref('')
const password = ref('')
const errorMessage = ref('')
const isLoading = ref(false)
const mode = ref<'login' | 'register'>('login')

async function handleSubmit() {
  errorMessage.value = ''
  isLoading.value = true
  
  try {
    if (mode.value === 'login') {
      await authStore.login(username.value, password.value)
    } else {
      await authStore.register(username.value, password.value)
      await authStore.login(username.value, password.value)
    }
    await router.push('/')
    username.value = ''
    password.value = ''
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : 'Login failed'
  } finally {
    isLoading.value = false
  }
}

function toggleMode() {
  mode.value = mode.value === 'login' ? 'register' : 'login'
  errorMessage.value = ''
}
</script>

<style scoped>
.login-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 300px;
  margin: 2rem auto;
  padding: 2rem;
  background-color: var(--color-surface);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
}

h2 {
  margin: 0;
  color: var(--color-text);
  text-align: center;
}

.error-message {
  padding: 0.75rem;
  background-color: rgba(239, 68, 68, 0.2);
  color: #fca5a5;
  border-radius: 0.25rem;
  font-size: 0.875rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

label {
  color: var(--color-text);
  font-weight: 600;
}

input[type="text"],
input[type="password"] {
  padding: 0.75rem;
  background-color: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: 0.25rem;
}

input[type="text"]:focus,
input[type="password"]:focus {
  outline: none;
  border-color: var(--color-primary);
}

.submit-btn {
  padding: 0.75rem;
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: 0.25rem;
  font-weight: 600;
  cursor: pointer;
}

.secondary-btn {
  padding: 0.5rem;
  background: transparent;
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: 0.25rem;
  cursor: pointer;
}

.submit-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.submit-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
