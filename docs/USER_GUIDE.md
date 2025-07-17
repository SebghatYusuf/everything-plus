# Everything Plus - User Guide

A modern file search application inspired by macOS Spotlight and Everything.

## Installation

### Windows

1. Download the latest release from [GitHub Releases](https://github.com/your-username/everything-plus/releases)
2. Run the installer `EverythingClone-Setup.exe`
3. Follow the installation wizard
4. Launch Everything Plus from the Start menu or desktop shortcut

### Manual Installation

1. Ensure you have the [Everything application](https://www.voidtools.com/) installed
2. Download and extract the portable version
3. Run `everything-plus.exe`

## Getting Started

### First Launch

On first launch, Everything Plus will:

1. **Initialize the database** - Creates a local SQLite database for indexing
2. **Start initial indexing** - Scans your selected drives/folders
3. **Begin file monitoring** - Watches for file system changes

### Basic Search

1. **Open the search window** - Click the application icon or use the global shortcut (default: `Ctrl+Space`)
2. **Type your search query** - Results appear instantly as you type
3. **Select a result** - Click to open the file/folder, or use arrow keys + Enter

## Search Features

### Simple Search

Just type what you're looking for:

```
readme
photo vacation
project.zip
```

### Advanced Filters

Click the filter button (🔧) to access advanced options:

#### File Types
- **Documents**: PDF, DOC, DOCX, TXT, MD
- **Images**: JPG, PNG, GIF, SVG, WebP
- **Videos**: MP4, AVI, MOV, WMV
- **Audio**: MP3, WAV, FLAC, AAC
- **Archives**: ZIP, RAR, 7Z, TAR
- **Code**: JS, TS, HTML, CSS, JSON, PY
- **Folders**: Directory/folder results only

#### File Size
- **Tiny**: < 10 KB
- **Small**: < 100 KB  
- **Medium**: < 1 MB
- **Large**: < 10 MB
- **Huge**: > 10 MB

#### Search Options
- **Case sensitive**: Exact case matching
- **Regular expressions**: Use regex patterns
- **Search file contents**: Search inside files (slower)
- **Include hidden files**: Show hidden/system files

### Search Examples

#### Basic Examples
```
vacation.jpg           # Find specific file
*.pdf                  # All PDF files
report 2024           # Files containing both words
```

#### With Filters
```
presentation          # + File Types: Documents
family                # + File Types: Images + Size: Large
config                # + Search Options: Include hidden files
```

#### Regular Expressions
Enable "Use regular expressions" for advanced patterns:
```
\d{4}-\d{2}-\d{2}     # Date pattern (2024-01-15)
test.*\.js$           # JavaScript files starting with "test"
^[A-Z].*\.txt         # Text files starting with capital letter
```

## Keyboard Shortcuts

### Global Shortcuts
- `Ctrl+Space` - Open search window (customizable)

### Search Window
- `Ctrl+K` - Focus search input
- `Ctrl+,` - Open settings
- `Escape` - Clear search or close window
- `↑/↓` - Navigate results
- `Enter` - Open selected result
- `Ctrl+Enter` - Open file location
- `F5` - Refresh results

### View Options
- `Ctrl+1` - List view
- `Ctrl+2` - Grid view
- `Ctrl+D` - Toggle dark/light theme

## Settings & Configuration

Access settings via `Ctrl+,` or the settings button (⚙️).

### Index Locations

**Add Folders to Index:**
1. Go to Settings → Index Locations
2. Click "Add new path..."
3. Enter the folder path (e.g., `D:\Documents`)
4. Click the + button

**Default Indexed Locations:**
- `C:\` (Windows system drive)
- Additional drives can be added manually

**Exclude Locations:**
Common exclusions (pre-configured):
- `C:\\Windows\WinSxS`
- `C:\\$Recycle.Bin`
- `.git` folders
- `node_modules` folders
- Build directories (`target`, `dist`, `build`)

### Performance Settings

**Maximum Results:** Control how many results are shown (default: 1000)

**Enable Network Drives:** Include network mapped drives in indexing
- ⚠️ May impact performance
- Recommended only for frequently accessed network locations

### System Integration

**Start with Windows:** Launch Everything Plus when Windows boots

**Show in System Tray:** Keep the application accessible from the system tray

**Global Shortcut:** Customize the keyboard shortcut to open search
- Default: `Ctrl+Space`
- Examples: `Ctrl+Alt+F`, `Win+S`, `F12`

## View Modes

### List View (Default)
- Compact file listing with details
- Shows file icon, name, path, size, and date
- Best for detailed file information
- Sortable columns

### Grid View
- Larger file icons in a grid layout
- Shows file type icons and basic info
- Best for visual file browsing
- Good for images and media files

## File Operations

### Opening Files
- **Single click** - Select file
- **Double click** - Open file with default application
- **Ctrl+Enter** - Open file location in Explorer

### Context Menu (Right-click)
- Open with default app
- Open file location
- Copy file path
- Properties
- *(Note: Context menu integration coming in future update)*

## Performance & Troubleshooting

### Performance Tips

1. **Exclude unnecessary folders** from indexing
2. **Limit network drive indexing** if experiencing slowdowns
3. **Reduce maximum results** if searches are slow
4. **Close other disk-intensive applications** during initial indexing

### Common Issues

#### Slow Search Results
- Check if indexing is still in progress
- Reduce the number of indexed locations
- Exclude large, unnecessary directories

#### High CPU/Memory Usage
- Usually occurs during initial indexing
- Check indexing progress in system tray tooltip
- Consider excluding system directories

#### Files Not Appearing in Results
- Ensure the file location is in indexed paths
- Check if file is hidden (enable "Include hidden files")
- Wait for real-time indexing to catch up (usually <1 second)

#### Application Won't Start
- Ensure Everything application is installed and running
- Check Windows Defender or antivirus exclusions
- Try running as administrator

### Database Maintenance

The application automatically maintains its database, but you can:

1. **Clear Index** - Settings → Advanced → Clear Index
   - Removes all indexed data
   - Next launch will re-index everything

2. **Database Location** - `%APPDATA%\EverythingClone\data\everything_clone.db`
   - Backup this file to preserve your index
   - Delete to reset application

## Advanced Usage

### Power User Tips

1. **Bookmark Frequent Searches** - Use browser bookmarks for common filters
2. **Combine Filters** - Mix file types, sizes, and dates for precise results  
3. **Use Regex** - Learn basic regex for powerful pattern matching
4. **Monitor System Tray** - Shows indexing status and quick access

### Command Line Interface

For developers and automation:

```bash
# Search from command line
everything-plus.exe --search "readme.md"

# Export results
everything-plus.exe --search "*.log" --export results.csv

# Rebuild index
everything-plus.exe --reindex
```

### Scripting Integration

Everything Plus can be integrated with scripts and other applications through its API endpoints (when running).

## Privacy & Security

- **Local Only**: All data stays on your computer
- **No Cloud Sync**: No data is sent to external servers
- **File Content**: Content search is optional and processed locally
- **Permissions**: Respects Windows file permissions

## Updates

Everything Plus checks for updates automatically and will notify you when new versions are available.

**Manual Update Check:**
1. Help → Check for Updates
2. Follow prompts to download and install

## Support

### Getting Help

1. **This User Guide** - Comprehensive feature documentation
2. **GitHub Issues** - Report bugs or request features
3. **Development Guide** - For developers and contributors

### Reporting Issues

When reporting issues, please include:
- Operating system version
- Everything Plus version
- Steps to reproduce the issue
- Any error messages
- Sample files/folders if relevant

### Feature Requests

Everything Plus is actively developed. Feature requests are welcome:
- GitHub Issues with "enhancement" label
- Include use case and expected behavior
- Check existing issues first

---

**Everything Plus v1.0** - Built with ❤️ for fast, modern file searching.
