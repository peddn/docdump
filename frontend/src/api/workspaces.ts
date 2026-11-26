import { getJson } from './client'
import type { Workspace } from './types'

/** Liste der konfigurierten Workspaces. */
export function fetchWorkspaces(): Promise<Workspace[]> {
  return getJson<Workspace[]>('/api/workspaces')
}
