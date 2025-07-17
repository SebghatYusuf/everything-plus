import { useState } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from './ui/dialog'
import { Button } from './ui/button'
import { AppSettings, Theme } from '../types'
import { Folder, Plus, Trash2, ToggleLeft, ToggleRight, Check } from 'lucide-react'
import { useTheme } from '../hooks/useTheme'

interface SettingsDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

const defaultSettings: AppSettings = {
  theme: 'dark',
  indexPaths: ['C:'],
  excludePaths: ['C:\Windows\WinSxS', 'C:\$Recycle.Bin'],
  maxResults: 1000,
  enableNetworkDrives: false,
  startWithWindows: false,
  showInSystemTray: true,
  globalShortcut: 'Ctrl+Space',
}

export function SettingsDialog({ open, onOpenChange }: SettingsDialogProps) {
  const { theme, applyTheme, themes } = useTheme();
  const [settings, setSettings] = useState<AppSettings>({ ...defaultSettings, theme: theme as Theme });
  const [newPath, setNewPath] = useState('')

  const updateSetting = <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ) => {
    setSettings(prev => ({ ...prev, [key]: value }))
  }

  const addIndexPath = () => {
    if (newPath.trim() && !settings.indexPaths.includes(newPath.trim())) {
      updateSetting('indexPaths', [...settings.indexPaths, newPath.trim()])
      setNewPath('')
    }
  }

  const removeIndexPath = (path: string) => {
    updateSetting('indexPaths', settings.indexPaths.filter(p => p !== path))
  }

  const addExcludePath = () => {
    if (newPath.trim() && !settings.excludePaths.includes(newPath.trim())) {
      updateSetting('excludePaths', [...settings.excludePaths, newPath.trim()])
      setNewPath('')
    }
  }

  const removeExcludePath = (path: string) => {
    updateSetting('excludePaths', settings.excludePaths.filter(p => p !== path))
  }

  const handleSave = () => {
    applyTheme(settings.theme);
    console.log('Saving settings:', settings)
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Settings</DialogTitle>
        </DialogHeader>

        <div className="space-y-6">
          {/* Theme */}
          <div>
            <h3 className="text-lg font-medium mb-3">Theme</h3>
            <div className="grid grid-cols-3 gap-4">
              {themes.map((t) => (
                <Button
                  key={t.name}
                  variant={settings.theme === t.name ? 'default' : 'outline'}
                  onClick={() => updateSetting('theme', t.name as Theme)}
                  className="h-16 flex flex-col items-start justify-between"
                >
                  <div className="flex items-center justify-between w-full">
                    <span className="capitalize">{t.name}</span>
                    {settings.theme === t.name && <Check className="w-4 h-4" />}
                  </div>
                  <div className="flex items-center gap-1">
                    <div className="w-4 h-4 rounded-full" style={{ backgroundColor: `hsl(${t.colors.primary})` }}></div>
                    <div className="w-4 h-4 rounded-full" style={{ backgroundColor: `hsl(${t.colors.secondary})` }}></div>
                    <div className="w-4 h-4 rounded-full" style={{ backgroundColor: `hsl(${t.colors.accent})` }}></div>
                  </div>
                </Button>
              ))}
            </div>
          </div>

          {/* Index Paths */}
          <div>
            <h3 className="text-lg font-medium mb-3">Index Locations</h3>
            <p className="text-sm text-muted-foreground mb-3">
              Choose which folders to include in the search index
            </p>
            
            <div className="space-y-2 mb-3">
              {settings.indexPaths.map((path) => (
                <div key={path} className="flex items-center gap-2 p-2 bg-muted rounded">
                  <Folder className="w-4 h-4" />
                  <span className="flex-1 text-sm">{path}</span>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => removeIndexPath(path)}
                  >
                    <Trash2 className="w-4 h-4" />
                  </Button>
                </div>
              ))}
            </div>

            <div className="flex gap-2">
              <input
                type="text"
                placeholder="Add new path..."
                value={newPath}
                onChange={(e) => setNewPath(e.target.value)}
                className="flex-1 px-3 py-2 border border-input rounded bg-background"
                onKeyDown={(e) => e.key === 'Enter' && addIndexPath()}
              />
              <Button onClick={addIndexPath}>
                <Plus className="w-4 h-4" />
              </Button>
            </div>
          </div>

          {/* Exclude Paths */}
          <div>
            <h3 className="text-lg font-medium mb-3">Exclude Locations</h3>
            <p className="text-sm text-muted-foreground mb-3">
              Folders to exclude from indexing
            </p>
            
            <div className="space-y-2 mb-3">
              {settings.excludePaths.map((path) => (
                <div key={path} className="flex items-center gap-2 p-2 bg-muted rounded">
                  <Folder className="w-4 h-4" />
                  <span className="flex-1 text-sm">{path}</span>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => removeExcludePath(path)}
                  >
                    <Trash2 className="w-4 h-4" />
                  </Button>
                </div>
              ))}
            </div>

            <div className="flex gap-2">
              <input
                type="text"
                placeholder="Add excluded path..."
                value={newPath}
                onChange={(e) => setNewPath(e.target.value)}
                className="flex-1 px-3 py-2 border border-input rounded bg-background"
                onKeyDown={(e) => e.key === 'Enter' && addExcludePath()}
              />
              <Button onClick={addExcludePath}>
                <Plus className="w-4 h-4" />
              </Button>
            </div>
          </div>

          {/* Performance Settings */}
          <div>
            <h3 className="text-lg font-medium mb-3">Performance</h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Maximum Results</p>
                  <p className="text-xs text-muted-foreground">
                    Limit the number of search results shown
                  </p>
                </div>
                <input
                  type="number"
                  value={settings.maxResults}
                  onChange={(e) => updateSetting('maxResults', parseInt(e.target.value) || 1000)}
                  className="w-20 px-2 py-1 border border-input rounded bg-background text-sm"
                  min="100"
                  max="10000"
                />
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Enable Network Drives</p>
                  <p className="text-xs text-muted-foreground">
                    Include network drives in indexing (may impact performance)
                  </p>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => updateSetting('enableNetworkDrives', !settings.enableNetworkDrives)}
                >
                  {settings.enableNetworkDrives ? (
                    <ToggleRight className="w-6 h-6 text-primary" />
                  ) : (
                    <ToggleLeft className="w-6 h-6 text-muted-foreground" />
                  )}
                </Button>
              </div>
            </div>
          </div>

          {/* System Integration */}
          <div>
            <h3 className="text-lg font-medium mb-3">System Integration</h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Start with Windows</p>
                  <p className="text-xs text-muted-foreground">
                    Launch Everything Plus when Windows starts
                  </p>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => updateSetting('startWithWindows', !settings.startWithWindows)}
                >
                  {settings.startWithWindows ? (
                    <ToggleRight className="w-6 h-6 text-primary" />
                  ) : (
                    <ToggleLeft className="w-6 h-6 text-muted-foreground" />
                  )}
                </Button>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Show in System Tray</p>
                  <p className="text-xs text-muted-foreground">
                    Keep the application accessible from the system tray
                  </p>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => updateSetting('showInSystemTray', !settings.showInSystemTray)}
                >
                  {settings.showInSystemTray ? (
                    <ToggleRight className="w-6 h-6 text-primary" />
                  ) : (
                    <ToggleLeft className="w-6 h-6 text-muted-foreground" />
                  )}
                </Button>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm font-medium">Global Shortcut</p>
                  <p className="text-xs text-muted-foreground">
                    Keyboard shortcut to open the search window
                  </p>
                </div>
                <input
                  type="text"
                  value={settings.globalShortcut}
                  onChange={(e) => updateSetting('globalShortcut', e.target.value)}
                  className="w-32 px-2 py-1 border border-input rounded bg-background text-sm"
                />
              </div>
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-2 pt-4">
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSave}>
            Save Settings
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}
