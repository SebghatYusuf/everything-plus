import { SearchFilters } from '../types'
import { Button } from './ui/button'
import { X, HardDrive, Filter } from 'lucide-react'

interface FilterPanelProps {
  filters: SearchFilters
  onChange: (filters: SearchFilters) => void
}

const FILE_TYPES = [
  { label: 'Documents', extensions: ['pdf', 'doc', 'docx', 'txt', 'md'] },
  { label: 'Images', extensions: ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp'] },
  { label: 'Videos', extensions: ['mp4', 'avi', 'mov', 'wmv', 'flv'] },
  { label: 'Audio', extensions: ['mp3', 'wav', 'flac', 'aac', 'ogg'] },
  { label: 'Archives', extensions: ['zip', 'rar', '7z', 'tar', 'gz'] },
  { label: 'Code', extensions: ['js', 'ts', 'html', 'css', 'json', 'py', 'java'] },
  { label: 'Folders', extensions: ['folder'] },
]

const SIZE_PRESETS = [
  { label: 'Tiny (< 10 KB)', max: 10 * 1024 },
  { label: 'Small (< 100 KB)', max: 100 * 1024 },
  { label: 'Medium (< 1 MB)', max: 1024 * 1024 },
  { label: 'Large (< 10 MB)', max: 10 * 1024 * 1024 },
  { label: 'Huge (> 10 MB)', min: 10 * 1024 * 1024 },
]

export function FilterPanel({ filters, onChange }: FilterPanelProps) {
  const updateFilters = (updates: Partial<SearchFilters>) => {
    onChange({ ...filters, ...updates })
  }

  const toggleFileType = (extensions: string[]) => {
    const newTypes = filters.fileTypes.includes(extensions[0])
      ? filters.fileTypes.filter(t => !extensions.includes(t))
      : [...filters.fileTypes, ...extensions]
    
    updateFilters({ fileTypes: newTypes })
  }

  const setSizeFilter = (preset: { min?: number; max?: number }) => {
    updateFilters({ 
      sizeMin: preset.min, 
      sizeMax: preset.max 
    })
  }

  const clearAllFilters = () => {
    onChange({
      fileTypes: [],
      includeHidden: false,
      caseSensitive: false,
      useRegex: false,
      searchContent: false,
    })
  }

  const hasActiveFilters = 
    filters.fileTypes.length > 0 ||
    filters.sizeMin !== undefined ||
    filters.sizeMax !== undefined ||
    filters.dateFrom !== undefined ||
    filters.dateTo !== undefined ||
    filters.includeHidden ||
    filters.caseSensitive ||
    filters.useRegex ||
    filters.searchContent

  return (
    <div className="bg-card border border-border rounded-lg p-4 space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Filter className="w-4 h-4" />
          <h3 className="font-medium">Filters</h3>
        </div>
        {hasActiveFilters && (
          <Button variant="ghost" size="sm" onClick={clearAllFilters}>
            <X className="w-4 h-4 mr-1" />
            Clear All
          </Button>
        )}
      </div>

      {/* File Types */}
      <div>
        <h4 className="text-sm font-medium mb-2">File Types</h4>
        <div className="flex flex-wrap gap-2">
          {FILE_TYPES.map((type) => (
            <Button
              key={type.label}
              variant={
                type.extensions.some(ext => filters.fileTypes.includes(ext))
                  ? "default"
                  : "outline"
              }
              size="sm"
              onClick={() => toggleFileType(type.extensions)}
            >
              {type.label}
            </Button>
          ))}
        </div>
      </div>

      {/* Size Filters */}
      <div>
        <h4 className="text-sm font-medium mb-2">File Size</h4>
        <div className="flex flex-wrap gap-2">
          {SIZE_PRESETS.map((preset) => (
            <Button
              key={preset.label}
              variant={
                (preset.min === filters.sizeMin && preset.max === filters.sizeMax) ||
                (preset.min && filters.sizeMin === preset.min && !preset.max && !filters.sizeMax) ||
                (preset.max && filters.sizeMax === preset.max && !preset.min && !filters.sizeMin)
                  ? "default"
                  : "outline"
              }
              size="sm"
              onClick={() => setSizeFilter(preset)}
            >
              <HardDrive className="w-3 h-3 mr-1" />
              {preset.label}
            </Button>
          ))}
        </div>
      </div>

      {/* Search Options */}
      <div>
        <h4 className="text-sm font-medium mb-2">Search Options</h4>
        <div className="space-y-2">
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={filters.caseSensitive}
              onChange={(e) => updateFilters({ caseSensitive: e.target.checked })}
              className="rounded border-border"
            />
            Case sensitive
          </label>
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={filters.useRegex}
              onChange={(e) => updateFilters({ useRegex: e.target.checked })}
              className="rounded border-border"
            />
            Use regular expressions
          </label>
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={filters.searchContent}
              onChange={(e) => updateFilters({ searchContent: e.target.checked })}
              className="rounded border-border"
            />
            Search file contents
          </label>
          <label className="flex items-center gap-2 text-sm">
            <input
              type="checkbox"
              checked={filters.includeHidden}
              onChange={(e) => updateFilters({ includeHidden: e.target.checked })}
              className="rounded border-border"
            />
            Include hidden files
          </label>
        </div>
      </div>
    </div>
  )
}
