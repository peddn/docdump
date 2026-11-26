/**
 * Alle Oberflächentexte an einer Stelle (E9: Deutsch, zentral abgelegt).
 * Komponenten verwenden nur Texte von hier, keine eigenen Zeichenketten.
 */
export const de = {
  workspaces: {
    title: 'Workspaces',
    loading: 'Workspaces werden geladen …',
    empty: 'Es ist kein Workspace konfiguriert.',
  },
  /** Fehlertexte nach `code` aus `{error: {code, message}}` bzw. vom Client. */
  errors: {
    network: 'Der Server ist nicht erreichbar.',
    invalid_response: 'Der Server hat eine ungültige Antwort geschickt.',
    not_found: 'Nicht gefunden.',
    internal: 'Interner Fehler im Server.',
    unknown: 'Unbekannter Fehler.',
  },
} as const
