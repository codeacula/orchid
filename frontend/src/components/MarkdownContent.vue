<template>
  <div v-html="sanitizedHtml"></div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import MarkdownIt from 'markdown-it'
import DOMPurify from 'dompurify'

defineOptions({
  name: 'MarkdownContent'
})

const props = defineProps<{
  content: string
}>()

const md = MarkdownIt()

const sanitizedHtml = computed(() => {
  const html = md.render(props.content)
  return DOMPurify.sanitize(html)
})
</script>
