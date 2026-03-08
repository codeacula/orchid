import { vi } from 'vitest'

// Mock DOMPurify
vi.mock('dompurify', () => ({
  default: {
    sanitize: (dirty: string) => {
      const div = document.createElement('div')
      div.innerHTML = dirty
      // Remove script tags
      const scripts = div.querySelectorAll('script')
      scripts.forEach(script => script.remove())
      return div.innerHTML
    }
  }
}))

// Mock markdown-it with shiki
vi.mock('markdown-it', () => {
  const md = {
    render: (text: string) => {
      let html = text
      // Simple markdown parsing for tests
      html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
      html = html.replace(/`(.+?)`/g, '<code>$1</code>')
      html = html.replace(/- (.+)/g, '<li>$1</li>')
      return html
    }
  }
  return {
    default: () => md
  }
})

// Mock window.matchMedia for jsdom (not available by default)
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn()
  }))
})
