import { afterEach, describe, expect, it, vi } from 'vitest'

import { ApiError, getJson } from '../client'

function mockFetch(impl: () => Promise<Response>) {
  vi.stubGlobal('fetch', vi.fn(impl))
}

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'Content-Type': 'application/json' },
  })
}

async function catchError(promise: Promise<unknown>): Promise<ApiError> {
  try {
    await promise
  } catch (error) {
    if (error instanceof ApiError) return error
    throw error
  }
  throw new Error('expected ApiError')
}

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('getJson', () => {
  it('liefert den JSON-Inhalt einer erfolgreichen Antwort', async () => {
    mockFetch(async () => jsonResponse([{ id: 'beruf', name: 'Beruf' }]))
    await expect(getJson('/api/workspaces')).resolves.toEqual([{ id: 'beruf', name: 'Beruf' }])
  })

  it('übernimmt code, message und Status aus einer Fehlerantwort', async () => {
    mockFetch(async () => jsonResponse({ error: { code: 'not_found', message: 'no route' } }, 404))
    const error = await catchError(getJson('/api/x'))
    expect(error.code).toBe('not_found')
    expect(error.message).toBe('no route')
    expect(error.status).toBe(404)
  })

  it('meldet unknown bei einer Fehlerantwort ohne Fehlerobjekt', async () => {
    mockFetch(async () => new Response('Bad Gateway', { status: 502 }))
    const error = await catchError(getJson('/api/x'))
    expect(error.code).toBe('unknown')
    expect(error.status).toBe(502)
  })

  it('meldet invalid_response bei Erfolg ohne JSON', async () => {
    mockFetch(async () => new Response('<html></html>', { status: 200 }))
    const error = await catchError(getJson('/api/x'))
    expect(error.code).toBe('invalid_response')
  })

  it('meldet network, wenn der Server nicht erreichbar ist', async () => {
    mockFetch(async () => {
      throw new TypeError('Failed to fetch')
    })
    const error = await catchError(getJson('/api/x'))
    expect(error.code).toBe('network')
    expect(error.status).toBeUndefined()
  })
})
