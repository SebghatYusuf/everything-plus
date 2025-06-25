import { useState, useEffect } from 'react'
import { Search, Settings, Moon, Sun, Grid, List } from 'lucide-react'
import { useTheme } from './hooks/useTheme'
import { useSearch } from './hooks/useSearch'
import { SearchInput } from './components/SearchInput'
import { SearchResults } from './components/SearchResults'
import { FilterPanel } from './components/FilterPanel'
import { SettingsDialog } from './components/SettingsDialog'
import { Button } from './components/ui/button'
import { TauriAPI, isTauri } from './lib/tauri'
import './App.css'

function App() {
  const { theme, toggleTheme } = useTheme()
  const { query, setQuery, results, filters, setFilters, viewMode, setViewMode, isLoading, backendReady } = useSearch()
  const [showSettings, setShowSettings] = useState(false)
  const [showFilters, setShowFilters] = useState(false)

  // Initialize Tauri app if running in desktop mode
  useEffect(() => {
    if (isTauri) {
      TauriAPI.initializeApp()
      
      // Listen for system tray settings requests
      const handleTauriOpenSettings = () => {
        setShowSettings(true)
      }
      
      window.addEventListener('tauri-open-settings', handleTauriOpenSettings)
      
      return () => {
        window.removeEventListener('tauri-open-settings', handleTauriOpenSettings)
      }
    }
  }, [])

  // Global keyboard shortcuts
  useEffect(() => {
    const handleKeydown = (e: KeyboardEvent) => {
      // Ctrl+K or Cmd+K to focus search
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault()
        const searchInput = document.querySelector('input[type="search"]') as HTMLInputElement
        searchInput?.focus()
      }
      
      // Ctrl+, to open settings
      if ((e.ctrlKey || e.metaKey) && e.key === ',') {
        e.preventDefault()
        setShowSettings(true)
      }
      
      // Escape to clear search or close dialogs
      if (e.key === 'Escape') {
        if (showSettings) {
          setShowSettings(false)
        } else if (showFilters) {
          setShowFilters(false)
        } else if (query) {
          setQuery('')
        }
      }
    }

    document.addEventListener('keydown', handleKeydown)
    return () => document.removeEventListener('keydown', handleKeydown)
  }, [query, setQuery, showSettings, showFilters])

  return (
    <div className={`min-h-screen bg-background text-foreground ${theme}`}>
      <div className="container mx-auto max-w-4xl p-4">
        {/* Header */}
        <header className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-2">
            <Search className="w-6 h-6 text-primary" />
            <h1 className="text-xl font-semibold">
              Everything Plus {isTauri && <span className="text-xs text-muted-foreground">(Desktop)</span>}
            </h1>
          </div>
          
          <div className="flex items-center gap-2">
            {/* View Mode Toggle */}
            <div className="flex items-center gap-1 p-1 bg-muted rounded-lg">
              <Button
                variant={viewMode === 'list' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setViewMode('list')}
              >
                <List className="w-4 h-4" />
              </Button>
              <Button
                variant={viewMode === 'grid' ? 'default' : 'ghost'}
                size="sm"
                onClick={() => setViewMode('grid')}
              >
                <Grid className="w-4 h-4" />
              </Button>
            </div>
            
            {/* Theme Toggle */}
            <Button variant="outline" size="sm" onClick={toggleTheme}>
              {theme === 'dark' ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
            </Button>
            
            {/* Settings */}
            <Button variant="outline" size="sm" onClick={() => setShowSettings(true)}>
              <Settings className="w-4 h-4" />
            </Button>
          </div>
        </header>

        {/* Search Input */}
        <div className="mb-6">
          <SearchInput
            value={query}
            onChange={setQuery}
            onToggleFilters={() => setShowFilters(!showFilters)}
            showFilters={showFilters}
            isLoading={isLoading}
            backendReady={isTauri ? backendReady : true}
          />
        </div>

        {/* Filter Panel */}
        {showFilters && (
          <div className="mb-6">
            <FilterPanel filters={filters} onChange={setFilters} />
          </div>
        )}

        {/* Backend Status Message */}
        {isTauri && !backendReady && (
          <div className="mb-6 p-4 bg-muted rounded-lg border">
            <div className="flex items-center gap-2 text-sm text-muted-foreground">
              <div className="w-4 h-4 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
              Initializing file system index... This may take a few moments.
            </div>
          </div>
        )}

        {/* Search Results */}
        <SearchResults
          results={results}
          query={query}
          viewMode={viewMode}
          isLoading={isLoading}
        />

        {/* Settings Dialog */}
        <SettingsDialog open={showSettings} onOpenChange={setShowSettings} />
      </div>
    </div>
  )
}

export default App
