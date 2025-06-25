import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import type { SearchFilters, SearchResult, IndexStats } from '../types'

// Backend filter format (matches Rust structs)
interface BackendSearchFilters {
  file_types: string[]
  size_min?: number
  size_max?: number
  date_from?: string
  date_to?: string
  include_hidden: boolean
  case_sensitive: boolean
  use_regex: boolean
  search_content: boolean
  directories_only: boolean
  files_only: boolean
}

export class TauriAPI {
  static async searchFiles(query: string, filters: BackendSearchFilters): Promise<SearchResult> {
    try {
      const result = await invoke<SearchResult>('search_files', {
        query,
        filters,
      })
      return result
    } catch (error) {
      console.error('Search failed:', error)
      throw new Error(`Search failed: ${error}`)
    }
  }

  static async getIndexStats(): Promise<IndexStats> {
    try {
      return await invoke<IndexStats>('get_index_stats')
    } catch (error) {
      console.error('Failed to get index stats:', error)
      throw new Error(`Failed to get index stats: ${error}`)
    }
  }

  static async checkBackendStatus(): Promise<boolean> {
    try {
      return await invoke<boolean>('check_backend_status')
    } catch (error) {
      console.error('Failed to check backend status:', error)
      return false
    }
  }

  static async openFile(path: string): Promise<void> {
    try {
      await invoke('open_file', { path })
    } catch (error) {
      console.error('Failed to open file:', error)
      throw new Error(`Failed to open file: ${error}`)
    }
  }

  static async openFileLocation(path: string): Promise<void> {
    try {
      await invoke('open_file_location', { path })
    } catch (error) {
      console.error('Failed to open file location:', error)
      throw new Error(`Failed to open file location: ${error}`)
    }
  }

  static async showWindow(): Promise<void> {
    try {
      await invoke('show_window')
    } catch (error) {
      console.error('Failed to show window:', error)
    }
  }

  static async hideWindow(): Promise<void> {
    try {
      await invoke('hide_window')
    } catch (error) {
      console.error('Failed to hide window:', error)
    }
  }

  static async setupEventListeners() {
    // Listen for global shortcut to focus search
    await listen('focus-search', () => {
      const searchInput = document.querySelector('input[type="search"]') as HTMLInputElement
      searchInput?.focus()
    })

    // Listen for system tray settings request
    await listen('open-settings', () => {
      // Dispatch custom event to open settings
      window.dispatchEvent(new CustomEvent('tauri-open-settings'))
    })

    // Prevent default window close behavior
    await appWindow.onCloseRequested(async (event) => {
      // Prevent the default close behavior
      event.preventDefault()
      // Hide the window instead
      await TauriAPI.hideWindow()
    })
  }

  static async initializeApp() {
    try {
      await this.setupEventListeners()
      console.log('Tauri app initialized successfully')
    } catch (error) {
      console.error('Failed to initialize Tauri app:', error)
    }
  }
}

// Utility function to convert frontend filters to backend format
export const convertFiltersToBackend = (frontendFilters: SearchFilters): BackendSearchFilters => ({
  file_types: frontendFilters.fileTypes,
  size_min: frontendFilters.sizeMin,
  size_max: frontendFilters.sizeMax,
  date_from: frontendFilters.dateFrom?.toISOString(),
  date_to: frontendFilters.dateTo?.toISOString(),
  include_hidden: frontendFilters.includeHidden,
  case_sensitive: frontendFilters.caseSensitive,
  use_regex: frontendFilters.useRegex,
  search_content: frontendFilters.searchContent,
  directories_only: false,
  files_only: false,
})

// Check if running in Tauri environment
export const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined

// Export window controls for Tauri
export const windowControls = {
  minimize: () => appWindow.minimize(),
  maximize: () => appWindow.toggleMaximize(),
  close: () => TauriAPI.hideWindow(), // Hide instead of close
}
