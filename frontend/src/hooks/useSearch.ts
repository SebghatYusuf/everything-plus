import { useState, useEffect, useCallback } from 'react'
import { debounce } from '../lib/utils'
import { TauriAPI, isTauri } from '../lib/tauri'
import type { FileResult, SearchFilters, ViewMode } from '../types'

// Mock data for browser development (when not in Tauri)
const mockResults: FileResult[] = [
  {
    id: '1',
    name: 'Project Documentation.pdf',
    path: 'C:\\Users\\Documents\\Project Documentation.pdf',
    size: 1024 * 1024 * 2.5, // 2.5MB
    modified: new Date('2024-01-15'),
    type: 'file',
    extension: 'pdf'
  },
  {
    id: '2',
    name: 'src',
    path: 'C:\\Users\\Projects\\myapp\\src',
    size: 0,
    modified: new Date('2024-01-20'),
    type: 'folder'
  },
  {
    id: '3',
    name: 'config.json',
    path: 'C:\\Users\\AppData\\config.json',
    size: 1024 * 5, // 5KB
    modified: new Date('2024-01-22'),
    type: 'file',
    extension: 'json'
  },
  {
    id: '4',
    name: 'image.png',
    path: 'C:\\Users\\Pictures\\image.png',
    size: 1024 * 1024 * 1.2, // 1.2MB
    modified: new Date('2024-01-18'),
    type: 'file',
    extension: 'png'
  },
  {
    id: '5',
    name: 'video.mp4',
    path: 'C:\\Users\\Videos\\video.mp4',
    size: 1024 * 1024 * 100, // 100MB
    modified: new Date('2024-01-10'),
    type: 'file',
    extension: 'mp4'
  }
]

export function useSearch() {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<FileResult[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [backendReady, setBackendReady] = useState(false)
  const [viewMode, setViewMode] = useState<ViewMode>('list')
  const [filters, setFilters] = useState<SearchFilters>({
    fileTypes: [],
    includeHidden: false,
    caseSensitive: false,
    useRegex: false,
    searchContent: false,
    directoriesOnly: false,
    filesOnly: false,
  })

  // Convert backend search filters to frontend format
  const convertFiltersToBackend = (frontendFilters: SearchFilters) => ({
    file_types: frontendFilters.fileTypes,
    size_min: frontendFilters.sizeMin,
    size_max: frontendFilters.sizeMax,
    date_from: frontendFilters.dateFrom?.toISOString(),
    date_to: frontendFilters.dateTo?.toISOString(),
    include_hidden: frontendFilters.includeHidden,
    case_sensitive: frontendFilters.caseSensitive,
    use_regex: frontendFilters.useRegex,
    search_content: frontendFilters.searchContent,
    directories_only: frontendFilters.directoriesOnly,
    files_only: frontendFilters.filesOnly,
  })

  // Convert backend results to frontend format
  const convertResultsFromBackend = (backendResults: any[]): FileResult[] => {
    return backendResults.map(result => ({
      id: result.id,
      name: result.name,
      path: result.path,
      size: result.size,
      modified: new Date(result.modified),
      type: result.is_directory ? 'folder' : 'file',
      extension: result.extension,
    }))
  }

  // Mock search function for browser development
  const mockSearch = async (searchQuery: string, searchFilters: SearchFilters) => {
    if (!searchQuery.trim()) {
      setResults([])
      setIsLoading(false)
      return
    }

    setIsLoading(true)
    
    try {
      // Simulate API call delay
      await new Promise(resolve => setTimeout(resolve, 100))
      
      // Filter mock results based on query
      const filteredResults = mockResults.filter(result => {
        const matchesQuery = searchFilters.caseSensitive
          ? result.name.includes(searchQuery)
          : result.name.toLowerCase().includes(searchQuery.toLowerCase())
        
        if (!matchesQuery) return false
        
        // Apply file type filters
        if (searchFilters.fileTypes.length > 0) {
          if (result.type === 'file') {
            return searchFilters.fileTypes.includes(result.extension || '')
          }
          return searchFilters.fileTypes.includes('folder')
        }
        
        return true
      })
      
      setResults(filteredResults)
    } catch (error) {
      console.error('Search error:', error)
      setResults([])
    } finally {
      setIsLoading(false)
    }
  }

  // Tauri search function
  const tauriSearch = async (searchQuery: string, searchFilters: SearchFilters) => {
    if (!searchQuery.trim()) {
      setResults([])
      setIsLoading(false)
      return
    }

    setIsLoading(true)
    
    try {
      // Check if backend is ready before searching
      if (!backendReady) {
        console.log('Backend not ready, checking status...')
        const isReady = await TauriAPI.checkBackendStatus()
        if (!isReady) {
          console.log('Backend still initializing, skipping search')
          setResults([])
          return
        }
        setBackendReady(true)
      }

      const backendFilters = convertFiltersToBackend(searchFilters)
      const result = await TauriAPI.searchFiles(searchQuery, backendFilters as any)
      const convertedResults = convertResultsFromBackend(result.entries)
      setResults(convertedResults)
    } catch (error) {
      console.error('Search error:', error)
      if (error instanceof Error && error.message.includes('still initializing')) {
        console.log('Backend still initializing, will retry when ready')
        setBackendReady(false)
      }
      setResults([])
    } finally {
      setIsLoading(false)
    }
  }

  // Debounced search function
  const debouncedSearch = useCallback(
    debounce(async (searchQuery: string, searchFilters: SearchFilters) => {
      if (isTauri) {
        await tauriSearch(searchQuery, searchFilters)
      } else {
        await mockSearch(searchQuery, searchFilters)
      }
    }, 300),
    [isTauri]
  )

  // Effect to trigger search when query or filters change
  useEffect(() => {
    debouncedSearch(query, filters)
  }, [query, filters, debouncedSearch])

  // Effect to set up backend ready listeners and initial status check
  useEffect(() => {
    if (!isTauri) return

    let unlisten: (() => void) | undefined

    const setupListeners = async () => {
      try {
        // Import listen dynamically to avoid issues in non-Tauri environments
        const { listen } = await import('@tauri-apps/api/event')
        
        // Listen for backend ready event
        unlisten = await listen('backend-ready', () => {
          console.log('Backend is ready!')
          setBackendReady(true)
          // Trigger search again if there's a query
          if (query.trim()) {
            debouncedSearch(query, filters)
          }
        })

        // Check initial backend status
        const isReady = await TauriAPI.checkBackendStatus()
        setBackendReady(isReady)
        console.log('Initial backend status:', isReady)
      } catch (error) {
        console.error('Failed to setup backend listeners:', error)
      }
    }

    setupListeners()

    // Cleanup
    return () => {
      if (unlisten) {
        unlisten()
      }
    }
  }, [query, filters, debouncedSearch])

  return {
    query,
    setQuery,
    results,
    filters,
    setFilters,
    viewMode,
    setViewMode,
    isLoading,
    backendReady,
  }
}
