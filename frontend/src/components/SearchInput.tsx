import { Search, Filter, Loader2 } from 'lucide-react'
import { Button } from './ui/button'

interface SearchInputProps {
  value: string
  onChange: (value: string) => void
  onToggleFilters: () => void
  showFilters: boolean
  isLoading: boolean
  backendReady?: boolean
}

export function SearchInput({ 
  value, 
  onChange, 
  onToggleFilters, 
  showFilters, 
  isLoading,
  backendReady = true
}: SearchInputProps) {
  const getPlaceholderText = () => {
    if (backendReady === false) {
      return "Initializing file index... Please wait..."
    }
    return "Search files and folders... (Ctrl+K)"
  }

  return (
    <div className="relative flex items-center gap-2">
      <div className="relative flex-1">
        <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <input
          type="search"
          placeholder={getPlaceholderText()}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          disabled={!backendReady}
          className="w-full pl-10 pr-4 py-3 text-lg border border-input rounded-lg bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          autoFocus={backendReady}
        />
        {(isLoading || !backendReady) && (
          <Loader2 className="absolute right-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground animate-spin" />
        )}
      </div>
      
      <Button
        variant={showFilters ? "default" : "outline"}
        size="lg"
        onClick={onToggleFilters}
        className="px-4"
        disabled={!backendReady}
      >
        <Filter className="w-4 h-4" />
      </Button>
    </div>
  )
}
