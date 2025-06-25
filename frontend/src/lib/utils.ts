import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes'
  
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

export function formatDate(date: Date): string {
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))
  
  if (diffDays === 0) {
    return 'Today'
  } else if (diffDays === 1) {
    return 'Yesterday'
  } else if (diffDays < 7) {
    return `${diffDays} days ago`
  } else {
    return date.toLocaleDateString()
  }
}

export function getFileIcon(extension: string | undefined, isFolder: boolean): string {
  if (isFolder) return '📁'
  
  if (!extension) return '📄'
  
  const iconMap: Record<string, string> = {
    // Documents
    'pdf': '📑',
    'doc': '📘',
    'docx': '📘',
    'txt': '📄',
    'md': '📝',
    'rtf': '📄',
    
    // Images
    'jpg': '🖼️',
    'jpeg': '🖼️',
    'png': '🖼️',
    'gif': '🖼️',
    'svg': '🖼️',
    'webp': '🖼️',
    'bmp': '🖼️',
    
    // Videos
    'mp4': '🎬',
    'avi': '🎬',
    'mov': '🎬',
    'wmv': '🎬',
    'flv': '🎬',
    'webm': '🎬',
    
    // Audio
    'mp3': '🎵',
    'wav': '🎵',
    'flac': '🎵',
    'aac': '🎵',
    'ogg': '🎵',
    
    // Archives
    'zip': '🗜️',
    'rar': '🗜️',
    '7z': '🗜️',
    'tar': '🗜️',
    'gz': '🗜️',
    
    // Code
    'js': '📜',
    'ts': '📜',
    'html': '🌐',
    'css': '🎨',
    'json': '📋',
    'xml': '📋',
    'py': '🐍',
    'java': '☕',
    'cpp': '⚙️',
    'c': '⚙️',
    'cs': '💻',
    'php': '💻',
    'rb': '💎',
    'go': '🔧',
    'rs': '🦀',
    
    // Executables
    'exe': '⚙️',
    'msi': '⚙️',
    'app': '📱',
    'deb': '📦',
    'rpm': '📦',
  }
  
  return iconMap[extension.toLowerCase()] || '📄'
}

export function highlightText(text: string, query: string): string {
  if (!query.trim()) return text
  
  const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return text.replace(regex, '<mark class="search-highlight">$1</mark>')
}

export function debounce<T extends (...args: any[]) => any>(
  func: T,
  waitFor: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout>
  return (...args: Parameters<T>): void => {
    clearTimeout(timeout)
    timeout = setTimeout(() => func(...args), waitFor)
  }
}
