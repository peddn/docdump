import { de } from './de'

/** Aktive Oberflächentexte. Vorerst nur Deutsch. */
export const texts = de

type ErrorCode = keyof typeof de.errors

/** Deutscher Text zu einem Fehlercode; unbekannte Codes ergeben einen allgemeinen Text. */
export function errorText(code: string): string {
  return Object.prototype.hasOwnProperty.call(texts.errors, code)
    ? texts.errors[code as ErrorCode]
    : texts.errors.unknown
}
