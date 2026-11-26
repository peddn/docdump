import { describe, expect, it } from 'vitest'

import { errorText, texts } from '..'

describe('errorText', () => {
  it('liefert den Text zu einem bekannten Code', () => {
    expect(errorText('network')).toBe(texts.errors.network)
  })

  it('liefert den allgemeinen Text zu einem unbekannten Code', () => {
    expect(errorText('gibt_es_nicht')).toBe(texts.errors.unknown)
  })

  it('fällt nicht auf Eigenschaften von Object.prototype herein', () => {
    expect(errorText('toString')).toBe(texts.errors.unknown)
  })
})
