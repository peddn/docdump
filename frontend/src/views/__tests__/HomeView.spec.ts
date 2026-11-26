import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { texts } from '@/texts'

import HomeView from '../HomeView.vue'

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('HomeView', () => {
  it('zeigt die Workspaces aus /api/workspaces', async () => {
    const fetchMock = vi.fn(async () =>
      Response.json([
        { id: 'beruf', name: 'Beruf' },
        { id: 'hobby', name: 'Hobby' },
      ]),
    )
    vi.stubGlobal('fetch', fetchMock)

    const wrapper = mount(HomeView, { global: { plugins: [createPinia()] } })
    await flushPromises()

    expect(fetchMock).toHaveBeenCalledWith('/api/workspaces', expect.anything())
    const items = wrapper.findAll('li').map((li) => li.text())
    expect(items).toEqual(['Beruf (beruf)', 'Hobby (hobby)'])
  })

  it('zeigt den deutschen Fehlertext, wenn der Server nicht erreichbar ist', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(async () => {
        throw new TypeError('Failed to fetch')
      }),
    )

    const wrapper = mount(HomeView, { global: { plugins: [createPinia()] } })
    await flushPromises()

    expect(wrapper.get('[role="alert"]').text()).toBe(texts.errors.network)
  })
})
