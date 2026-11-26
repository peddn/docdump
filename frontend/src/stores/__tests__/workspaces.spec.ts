import { createPinia, setActivePinia } from 'pinia'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import { useWorkspacesStore } from '../workspaces'

beforeEach(() => {
  setActivePinia(createPinia())
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('workspaces-Store', () => {
  it('lädt die Liste der Workspaces', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(async () => Response.json([{ id: 'beruf', name: 'Beruf' }])),
    )
    const store = useWorkspacesStore()
    const loading = store.load()
    expect(store.loading).toBe(true)
    await loading
    expect(store.loading).toBe(false)
    expect(store.errorCode).toBeNull()
    expect(store.list).toEqual([{ id: 'beruf', name: 'Beruf' }])
  })

  it('merkt sich den Fehlercode und leert die Liste', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(async () =>
        Response.json({ error: { code: 'internal', message: 'boom' } }, { status: 500 }),
      ),
    )
    const store = useWorkspacesStore()
    store.list = [{ id: 'alt', name: 'Alt' }]
    await store.load()
    expect(store.loading).toBe(false)
    expect(store.errorCode).toBe('internal')
    expect(store.list).toEqual([])
  })
})
