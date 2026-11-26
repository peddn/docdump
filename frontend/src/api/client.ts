/**
 * Fehler einer API-Anfrage.
 *
 * `code` stammt aus der Fehlerantwort `{error: {code, message}}` des Servers oder ist
 * ein eigener Code des Clients (`network`, `invalid_response`, `unknown`). Angezeigt
 * wird immer der deutsche Text zu `code`, nie `message`.
 */
export class ApiError extends Error {
  readonly code: string
  readonly status: number | undefined

  constructor(code: string, message: string, status?: number) {
    super(message)
    this.name = 'ApiError'
    this.code = code
    this.status = status
  }
}

interface ErrorBody {
  error: { code: string; message: string }
}

function isErrorBody(value: unknown): value is ErrorBody {
  if (typeof value !== 'object' || value === null || !('error' in value)) return false
  const error = value.error
  return (
    typeof error === 'object' &&
    error !== null &&
    'code' in error &&
    typeof error.code === 'string' &&
    'message' in error &&
    typeof error.message === 'string'
  )
}

/** Liest JSON von `url`. Wirft bei jedem Fehler einen `ApiError`. */
export async function getJson<T>(url: string): Promise<T> {
  let response: Response
  try {
    response = await fetch(url, { headers: { Accept: 'application/json' } })
  } catch (cause) {
    throw new ApiError('network', cause instanceof Error ? cause.message : String(cause))
  }

  let body: unknown
  try {
    body = await response.json()
  } catch {
    body = undefined
  }

  if (!response.ok) {
    if (isErrorBody(body)) {
      throw new ApiError(body.error.code, body.error.message, response.status)
    }
    throw new ApiError('unknown', `HTTP ${response.status}`, response.status)
  }
  if (body === undefined) {
    throw new ApiError('invalid_response', 'response body is not JSON', response.status)
  }
  return body as T
}
