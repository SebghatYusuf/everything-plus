---
applyTo: '**'
---
You're building a modern file search application like macos spotlight inspired by Everything with the following specifications:

Technical Requirements:
- Frontend: React 18+ with TypeScript
- Backend: Rust implementation using the official Everything SDK
- Database: SQLite for file/folder index storage
- UI Framework: Tailwind CSS v4.x with Shadcn/UI components
- Performance: WebAssembly modules for search operations

Core Features:
1. Real-time file/folder indexing
   - Instant indexing of NTFS file systems
   - Support for network drives
   - Configurable index locations

2. High-Performance Search
   - Instant results as you type
   - Advanced filters (size, date, type)
   - Regular expression support
   - Case-sensitive/insensitive options

3. Modern User Interface
   - Clean, minimal design
   - Dark/light theme support
   - Responsive layout
   - Customizable results view (list/grid)
   - Keyboard shortcuts

4. System Integration
   - Windows context menu integration
   - Global keyboard shortcuts
   - System tray presence
   - Start with Windows option

Performance Constraints:
- Maximum memory usage: 50MB idle
- Search response time: <100ms
- Index update latency: <1s

Documentation Requirements:
- User guide with examples
- Development setup guide

Reference Implementation:
- Everything SDK: https://www.voidtools.com/support/everything/sdk/
- File system monitoring: https://docs.rs/notify/
- SQLite optimization: https://www.sqlite.org/optimization.html

Testing Requirements:
- Unit tests for core functionality
- Integration tests for SDK interaction
- UI component tests
- Performance benchmarks