export interface FileResult {
  id: string
  name: string
  path: string
  size: number
  modified: Date
  type: 'file' | 'folder'
  extension?: string
  icon?: string
}

export interface SearchFilters {
  fileTypes: string[]
  sizeMin?: number
  sizeMax?: number
  dateFrom?: Date
  dateTo?: Date
  includeHidden: boolean
  caseSensitive: boolean
  useRegex: boolean
  searchContent: boolean
}

export interface SearchResult {
  entries: FileResult[]
  total_count: number
  query_time_ms: number
}

export interface IndexStats {
  total_files: number
  total_directories: number
  total_size: number
  last_updated: Date
  indexed_paths: string[]
}

export interface SearchOptions {
  query: string
  filters: SearchFilters
  maxResults: number
  sortBy: 'name' | 'size' | 'modified' | 'relevance'
  sortOrder: 'asc' | 'desc'
}

export type ViewMode = 'list' | 'grid'
export type Theme = 'light' | 'dark'

export interface AppSettings {
  theme: Theme
  indexPaths: string[]
  excludePaths: string[]
  maxResults: number
  enableNetworkDrives: boolean
  startWithWindows: boolean
  showInSystemTray: boolean
  globalShortcut: string
}
