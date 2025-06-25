import { FileResult, ViewMode } from '../types'
import { getFileIcon, formatFileSize, formatDate, highlightText } from '../lib/utils'
import { TauriAPI, isTauri } from '../lib/tauri'
import { FileText, Folder, ExternalLink } from 'lucide-react'

interface SearchResultsProps {
  results: FileResult[]
  query: string
  viewMode: ViewMode
  isLoading: boolean
}

export function SearchResults({ results, query, viewMode, isLoading }: SearchResultsProps) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
          <p className="text-muted-foreground">Searching...</p>
        </div>
      </div>
    )
  }

  if (!query.trim()) {
    return (
      <div className="text-center py-12">
        <FileText className="w-12 h-12 text-muted-foreground mx-auto mb-4" />
        <h3 className="text-lg font-medium mb-2">Start typing to search</h3>
        <p className="text-muted-foreground">
          Search for files and folders by name, type, or content
        </p>
      </div>
    )
  }

  if (results.length === 0) {
    return (
      <div className="text-center py-12">
        <FileText className="w-12 h-12 text-muted-foreground mx-auto mb-4" />
        <h3 className="text-lg font-medium mb-2">No results found</h3>
        <p className="text-muted-foreground">
          Try adjusting your search terms or filters
        </p>
      </div>
    )
  }

  const handleItemClick = async (result: FileResult) => {
    if (isTauri) {
      try {
        // Open the file or folder directly
        await TauriAPI.openFile(result.path)
      } catch (error) {
        console.error('Failed to open file:', error)
        // Show error to user
        alert(`Failed to open file: ${error}`)
      }
    } else {
      console.log('Opening:', result.path)
    }
  }

  const handleItemRightClick = async (result: FileResult, e: React.MouseEvent) => {
    e.preventDefault()
    if (isTauri) {
      try {
        // Open the file location (folder containing the file, or the folder itself)
        await TauriAPI.openFileLocation(result.path)
      } catch (error) {
        console.error('Failed to open file location:', error)
        // Show error to user
        alert(`Failed to open file location: ${error}`)
      }
    } else {
      console.log('Context menu for:', result.path)
    }
  }

  if (viewMode === 'grid') {
    return (
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {results.map((result) => (        <div
          key={result.id}
          className="p-4 border border-border rounded-lg hover:bg-accent/50 cursor-pointer transition-colors group"
          onClick={() => handleItemClick(result)}
          onContextMenu={(e) => handleItemRightClick(result, e)}
          title={`Click to open • Right-click to show in folder\n${result.path}`}
        >
            <div className="flex flex-col items-center text-center">
              <div className="text-4xl mb-2">
                {getFileIcon(result.extension, result.type === 'folder')}
              </div>
              <h4 
                className="font-medium text-sm mb-1 truncate w-full"
                dangerouslySetInnerHTML={{ 
                  __html: highlightText(result.name, query) 
                }}
              />
              <p className="text-xs text-muted-foreground truncate w-full mb-1">
                {result.path}
              </p>
              <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <span>{formatFileSize(result.size)}</span>
                <span>•</span>
                <span>{formatDate(result.modified)}</span>
              </div>
            </div>
          </div>
        ))}
      </div>
    )
  }

  return (
    <div className="space-y-1">
      {results.map((result) => (
        <div
          key={result.id}
          className="result-item group"
          onClick={() => handleItemClick(result)}
          onContextMenu={(e) => handleItemRightClick(result, e)}
          title={`Click to open • Right-click to show in folder\n${result.path}`}
        >
          <div className="file-icon text-xl">
            {getFileIcon(result.extension, result.type === 'folder')}
          </div>
          
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2 mb-1">
              <h4 
                className="font-medium truncate"
                dangerouslySetInnerHTML={{ 
                  __html: highlightText(result.name, query) 
                }}
              />
              {result.type === 'folder' && <Folder className="w-4 h-4 text-muted-foreground" />}
            </div>
            <p className="text-sm text-muted-foreground truncate">
              {result.path}
            </p>
          </div>
          
          <div className="flex items-center gap-4 text-sm text-muted-foreground">
            <span>{formatFileSize(result.size)}</span>
            <span>{formatDate(result.modified)}</span>
            <ExternalLink className="w-4 h-4 opacity-0 group-hover:opacity-100 transition-opacity" />
          </div>
        </div>
      ))}
    </div>
  )
}
