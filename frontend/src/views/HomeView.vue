<script setup lang="ts">
import { onMounted } from 'vue'

import { APP_NAME } from '@/constants'
import { useWorkspacesStore } from '@/stores/workspaces'
import { errorText, texts } from '@/texts'

const workspaces = useWorkspacesStore()

onMounted(() => {
  void workspaces.load()
})
</script>

<template>
  <main class="mx-auto max-w-xl p-8 font-mono">
    <h1 class="text-2xl font-semibold">{{ APP_NAME }}</h1>
    <h2 class="mt-6 text-sm text-neutral-500 uppercase">{{ texts.workspaces.title }}</h2>

    <p v-if="workspaces.loading" class="mt-2">{{ texts.workspaces.loading }}</p>
    <p v-else-if="workspaces.errorCode" class="mt-2 text-red-600 dark:text-red-400" role="alert">
      {{ errorText(workspaces.errorCode) }}
    </p>
    <p v-else-if="workspaces.list.length === 0" class="mt-2">{{ texts.workspaces.empty }}</p>
    <ul v-else class="mt-2 space-y-1">
      <li v-for="ws in workspaces.list" :key="ws.id">
        {{ ws.name }} <span class="text-neutral-500">({{ ws.id }})</span>
      </li>
    </ul>
  </main>
</template>
