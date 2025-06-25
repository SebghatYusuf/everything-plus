# Everything Clone - Modern File Search Application

A high-performance file search application inspired by macOS Spotlight and Everything, built with modern technologies.

## Features

- **Real-time file/folder indexing** - Instant indexing of NTFS file systems and network drives
- **High-performance search** - Sub-100ms search responses with advanced filtering
- **Modern UI** - Clean, responsive interface with dark/light themes
- **System integration** - Windows context menu, global shortcuts, system tray
- **Advanced filtering** - Size, date, type filters with regex support

## Tech Stack

- **Frontend**: React 18+ with TypeScript
- **Backend**: Rust with Everything SDK integration
- **Database**: SQLite for file indexing
- **UI**: Tailwind CSS v4.x + Shadcn/UI
- **Performance**: WebAssembly for search operations

## Quick Start

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Everything application installed (for SDK)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/everything-clone.git
cd everything-clone

# Install frontend dependencies
cd frontend
npm install

# Build Rust backend
cd ../backend
cargo build --release

# Run the application
npm run dev
```

## Project Structure

```
everything-clone/
├── frontend/          # React TypeScript frontend
├── backend/           # Rust backend with Everything SDK
├── shared/           # Shared types and utilities
├── docs/             # Documentation
└── tests/            # Integration tests
```

## Performance Goals

- Memory usage: <50MB idle
- Search response: <100ms
- Index update: <1s latency

## License

MIT License - see [LICENSE](LICENSE) for details.
