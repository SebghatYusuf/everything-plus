# Development Setup Guide

This guide will help you set up the Everything Clone development environment.

## Prerequisites

### Required Software

1. **Node.js 18+**
   - Download from [nodejs.org](https://nodejs.org/)
   - Verify installation: `node --version`

2. **Rust 1.70+**
   - Install via [rustup.rs](https://rustup.rs/)
   - Verify installation: `rustc --version`

3. **Everything Application (Windows)**
   - Download from [voidtools.com](https://www.voidtools.com/)
   - Required for SDK integration on Windows

### Optional Tools

- **Git** for version control
- **VS Code** with recommended extensions:
  - Rust Analyzer
  - TypeScript and JavaScript Language Features
  - Tailwind CSS IntelliSense
  - ES7+ React/Redux/React-Native snippets

## Project Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/everything-clone.git
cd everything-clone
```

### 2. Install Dependencies

```bash
# Install root dependencies
npm install

# Install frontend dependencies
cd frontend
npm install
cd ..

# Build Rust backend (first time)
cd backend
cargo build
cd ..
```

### 3. Database Setup

The application will automatically create a SQLite database on first run:

```bash
# Default location: ./data/everything_clone.db
mkdir -p data
```

### 4. Environment Configuration

Create a `.env` file in the root directory:

```env
# Development settings
NODE_ENV=development
VITE_API_URL=http://localhost:8080
RUST_LOG=debug
DATABASE_URL=./data/everything_clone.db

# Production settings (for deployment)
# NODE_ENV=production
# VITE_API_URL=https://your-domain.com/api
# RUST_LOG=info
```

## Development Workflow

### Running the Application

```bash
# Start both frontend and backend (recommended)
npm run dev

# Or start them separately:
npm run dev:frontend  # Frontend only (port 3000)
npm run dev:backend   # Backend only (port 8080)
```

### Building for Production

```bash
# Build everything
npm run build

# Or build separately:
npm run build:frontend
npm run build:backend
```

### Running Tests

```bash
# Run all tests
npm test

# Run frontend tests only
npm run test:frontend

# Run backend tests only
npm run test:backend
```

### Linting and Code Quality

```bash
# Lint all code
npm run lint

# Lint frontend only
npm run lint:frontend

# Lint backend only (Clippy)
npm run lint:backend
```

## Architecture Overview

### Frontend (React + TypeScript)

```
frontend/
├── src/
│   ├── components/          # React components
│   │   ├── ui/             # Shadcn/UI components
│   │   ├── SearchInput.tsx
│   │   ├── SearchResults.tsx
│   │   ├── FilterPanel.tsx
│   │   └── SettingsDialog.tsx
│   ├── hooks/              # Custom React hooks
│   │   ├── useTheme.ts
│   │   └── useSearch.ts
│   ├── lib/                # Utility functions
│   │   └── utils.ts
│   ├── types/              # TypeScript type definitions
│   │   └── index.ts
│   ├── App.tsx             # Main app component
│   ├── main.tsx           # Entry point
│   └── index.css          # Global styles
├── index.html
├── vite.config.ts
├── tailwind.config.js
└── package.json
```

### Backend (Rust)

```
backend/
├── src/
│   ├── database.rs         # SQLite database operations
│   ├── indexer.rs          # File system indexing
│   ├── search.rs           # Search engine logic
│   ├── types.rs            # Shared type definitions
│   ├── lib.rs              # Library interface
│   └── main.rs             # CLI application
├── Cargo.toml
└── README.md
```

## Key Features Implementation

### 1. File Indexing

The indexer uses Rust's `notify` crate for file system watching:

- **Initial scan**: Recursively indexes specified directories
- **Real-time updates**: Watches for file changes using OS-native APIs
- **Performance**: Batch inserts for optimal database performance

### 2. Search Engine

SQLite-based search with optimizations:

- **FTS5**: Full-text search for content searching
- **Indexes**: Multiple indexes for fast filtering
- **WAL mode**: Write-Ahead Logging for better concurrency

### 3. Frontend Architecture

Modern React patterns:

- **Custom hooks**: `useTheme`, `useSearch` for state management
- **Component composition**: Shadcn/UI for consistent design
- **TypeScript**: Full type safety across the application

## Performance Considerations

### Database Optimization

```sql
-- Applied automatically on startup
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = 10000;
PRAGMA temp_store = memory;
PRAGMA mmap_size = 268435456; -- 256MB
```

### Memory Usage

- Target: <50MB idle memory usage
- Batch processing for large directory scans
- Configurable result limits

### Search Performance

- Target: <100ms search response time
- Optimized SQL queries with proper indexing
- Debounced search input (300ms)

## Debugging

### Frontend Debugging

```bash
# Enable React DevTools
npm run dev

# Browser developer tools
# - Network tab for API calls
# - Console for React errors
# - Components tab (with React DevTools)
```

### Backend Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Or in VS Code with Rust Analyzer:
# Set breakpoints and use integrated debugger
```

### Database Debugging

```bash
# Connect to SQLite database
sqlite3 ./data/everything_clone.db

# Useful queries:
.schema                              # Show table structure
SELECT COUNT(*) FROM file_entries;   # Count indexed files
.exit
```

## Common Issues

### Windows-Specific

1. **Everything SDK**: Ensure Everything is installed and running
2. **File permissions**: Run as administrator if indexing system directories
3. **Path separators**: Code handles both `/` and `\\` automatically

### Performance Issues

1. **Slow indexing**: Check exclude patterns, avoid system directories
2. **High memory usage**: Reduce batch size in indexer
3. **Slow searches**: Check database indexes with `.explain query plan`

## Contributing

1. **Code style**: Follow existing patterns, use Prettier/Rustfmt
2. **Testing**: Add tests for new features
3. **Documentation**: Update this guide for new setup steps
4. **Performance**: Profile changes that affect search/indexing speed

## Deployment

See the main README.md for deployment instructions.

For questions or issues, please check the GitHub issues or create a new one.
