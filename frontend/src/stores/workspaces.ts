import { defineStore } from 'pinia'
import { ref } from 'vue'

import { ApiError } from '@/api/client'
import type { Workspace } from '@/api/types'
import { fetchWorkspaces } from '@/api/workspaces'

/** Konfigurierte Workspaces. Aktueller Workspace und dessen Konfiguration folgen mit M1. */
export const useWorkspacesStore = defineStore('workspaces', () => {
  const list = ref<Workspace[]>([])
  const loading = ref(false)
  /** Fehlercode der letzten Anfrage, `null` bei Erfolg. */
  const errorCode = ref<string | null>(null)

  async function load(): Promise<void> {
    loading.value = true
    errorCode.value = null
    try {
      list.value = await fetchWorkspaces()
    } catch (error) {
      list.value = []
      errorCode.value = error instanceof ApiError ? error.code : 'unknown'
    } finally {
      loading.value = false
    }
  }

  return { list, loading, errorCode, load }
})
