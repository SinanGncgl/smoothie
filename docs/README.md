# Smoothie - Desktop Workspace Profile Manager

A Tauri-based desktop application for managing and switching between different workspace profiles on macOS. Capture monitor arrangements, window positions, and running applications to create reusable workspace configurations.

## 🏗️ Architecture Overview

Smoothie is built with a **Tauri 2.0** framework, combining:
- **Frontend**: Next.js 16 with React 19, TypeScript, and Tailwind CSS
- **Backend**: Rust with SQLite database
- **UI Framework**: Radix UI components with shadcn/ui
- **Platform**: macOS-only (uses CoreGraphics APIs)

### Core Components

```
├── Frontend (Next.js + React)
│   ├── app/ - Next.js App Router pages
│   ├── components/ - Reusable UI components
│   ├── hooks/ - React hooks for state management
│   └── lib/ - Utility functions and Tauri API wrappers
├── Backend (Rust + Tauri)
│   ├── handlers/ - Tauri command handlers (API endpoints)
│   ├── services/ - Business logic layer
│   ├── repositories/ - Database access layer
│   ├── models/ - Data structures (DTOs and entities)
│   ├── db/ - Database connection and migrations
│   └── security/ - Authentication and middleware
└── Configuration
    ├── tauri.conf.json - Tauri app configuration
    ├── Cargo.toml - Rust dependencies
    └── package.json - Node.js dependencies
```

## 🚀 Key Features

- **System Detection**: Capture current monitor setup, window positions, and running apps
- **Profile Management**: Save and restore workspace configurations
- **Automation Rules**: Trigger profile switches based on conditions
- **Cross-Platform Sync**: Backup and restore profiles across devices
- **Modern UI**: Clean, responsive interface built with Radix UI

## 📁 File Structure & Documentation

### Frontend Files

#### Core Application Files
- [`app/layout.tsx`](docs/frontend/layout.md) - Root layout component
- [`app/page.tsx`](docs/frontend/page.md) - Main dashboard page
- [`app/globals.css`](docs/frontend/globals.md) - Global styles and Tailwind imports

#### Components
- [`components/system-capture.tsx`](docs/components/system-capture.md) - System capture interface
- [`components/sidebar.tsx`](docs/components/sidebar.md) - Main navigation sidebar
- [`components/top-bar.tsx`](docs/components/top-bar.md) - Top navigation bar
- [`components/dashboard.tsx`](docs/components/dashboard.md) - Main dashboard view
- [`components/profile-manager.tsx`](docs/components/profile-manager.md) - Profile CRUD interface
- [`components/monitor-editor.tsx`](docs/components/monitor-editor.md) - Monitor configuration editor
- [`components/settings.tsx`](docs/components/settings.md) - Application settings
- [`components/theme-provider.tsx`](docs/components/theme-provider.md) - Theme management
- [`components/navbar.tsx`](docs/components/navbar.md) - Navigation component

#### UI Components (shadcn/ui)
Located in `components/ui/` - Reusable UI primitives built on Radix UI

#### Hooks
- [`hooks/use-mobile.ts`](docs/hooks/use-mobile.md) - Mobile device detection
- [`hooks/use-toast.ts`](docs/hooks/use-toast.md) - Toast notification system
- [`hooks/use-system-detection.ts`](docs/hooks/use-system-detection.md) - System detection state management

#### Utilities
- [`lib/utils.ts`](docs/lib/utils.md) - General utility functions
- [`lib/tauri.ts`](docs/lib/tauri.md) - Tauri API wrapper and type definitions

### Backend Files

#### Entry Points
- [`src/main.rs`](docs/backend/main.md) - Application entry point and Tauri setup
- [`src/lib.rs`](docs/backend/lib.md) - Library module declarations

#### Database Layer
- [`src/db/connection.rs`](docs/backend/db/connection.md) - Database connection management
- [`src/db/migrations.rs`](docs/backend/db/migrations.md) - Database schema migrations
- [`src/db/mod.rs`](docs/backend/db/mod.md) - Database module exports
- [`src/db/queries.rs`](docs/backend/db/queries.md) - Optimized database queries

#### Handlers (API Endpoints)
- [`src/handlers/mod.rs`](docs/backend/handlers/mod.md) - Handler module exports
- [`src/handlers/profile.rs`](docs/backend/handlers/profile.md) - Profile CRUD operations
- [`src/handlers/monitor.rs`](docs/backend/handlers/monitor.md) - Monitor management
- [`src/handlers/app.rs`](docs/backend/handlers/app.md) - Application management
- [`src/handlers/browser.rs`](docs/backend/handlers/browser.md) - Browser tab management
- [`src/handlers/automation.rs`](docs/backend/handlers/automation.md) - Automation rules
- [`src/handlers/sync.rs`](docs/backend/handlers/sync.md) - Data synchronization
- [`src/handlers/user.rs`](docs/backend/handlers/user.md) - User preferences
- [`src/handlers/system.rs`](docs/backend/handlers/system.md) - System detection

#### Services (Business Logic)
- [`src/services/mod.rs`](docs/backend/services/mod.md) - Service module exports
- [`src/services/app_service.rs`](docs/backend/services/app-service.md) - Application business logic
- [`src/services/automation_service.rs`](docs/backend/services/automation-service.md) - Automation rules logic
- [`src/services/browser_service.rs`](docs/backend/services/browser-service.md) - Browser tab logic
- [`src/services/monitor_service.rs`](docs/backend/services/monitor-service.md) - Monitor management logic
- [`src/services/profile_service.rs`](docs/backend/services/profile-service.md) - Profile management logic
- [`src/services/sync_service.rs`](docs/backend/services/sync-service.md) - Synchronization logic
- [`src/services/system_service.rs`](docs/backend/services/system-service.md) - System detection (macOS)
- [`src/services/window_service.rs`](docs/backend/services/window-service.md) - Window management logic

#### Repositories (Data Access)
- [`src/repositories/mod.rs`](docs/backend/repositories/mod.md) - Repository module exports
- [`src/repositories/app_repository.rs`](docs/backend/repositories/app-repository.md) - App data access
- [`src/repositories/automation_repository.rs`](docs/backend/repositories/automation-repository.md) - Automation data access
- [`src/repositories/browser_tab_repository.rs`](docs/backend/repositories/browser-tab-repository.md) - Browser tab data access
- [`src/repositories/monitor_repository.rs`](docs/backend/repositories/monitor-repository.md) - Monitor data access
- [`src/repositories/profile_repository.rs`](docs/backend/repositories/profile-repository.md) - Profile data access
- [`src/repositories/sync_repository.rs`](docs/backend/repositories/sync-repository.md) - Sync data access

#### Models (Data Structures)
- [`src/models/mod.rs`](docs/backend/models/mod.md) - Model module exports
- [`src/models/dto.rs`](docs/backend/models/dto.md) - API Data Transfer Objects
- [`src/models/entities.rs`](docs/backend/models/entities.md) - Database entities

#### Security & Authentication
- [`src/security/mod.rs`](docs/backend/security/mod.md) - Security module exports
- [`src/security/auth.rs`](docs/backend/security/auth.md) - Authentication logic
- [`src/security/middleware.rs`](docs/backend/security/middleware.md) - Request middleware

#### Logging & Monitoring
- [`src/logging/mod.rs`](docs/backend/logging/mod.md) - Logging module exports
- [`src/logging/logger.rs`](docs/backend/logging/logger.md) - Structured logging
- [`src/logging/metrics.rs`](docs/backend/logging/metrics.md) - Application metrics
- [`src/logging/traces.rs`](docs/backend/logging/traces.md) - Request tracing

#### State Management
- [`src/state/mod.rs`](docs/backend/state/mod.md) - Application state management

#### Utilities
- [`src/utils/mod.rs`](docs/backend/utils/mod.md) - Utility module exports
- [`src/utils/encryption.rs`](docs/backend/utils/encryption.md) - Data encryption utilities
- [`src/utils/validation.rs`](docs/backend/utils/validation.md) - Input validation

#### Error Handling
- [`src/error/mod.rs`](docs/backend/error/mod.md) - Error types and handling

## 🔧 Development Setup

### Prerequisites
- Node.js 18+
- Rust 1.70+
- SQLite (bundled with the application)
- macOS (for system detection features)

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd smoothie

# Install Node.js dependencies
pnpm install

# Install Rust dependencies
cd src-tauri
cargo build

# Database is automatically set up with SQLite (no configuration needed)

# Start development server
cd ..
pnpm run dev
```

### Key Scripts
- `pnpm run dev` - Start development server
- `pnpm run build` - Build for production
- `cargo build` - Build Rust backend
- `cargo test` - Run Rust tests

## 📊 Data Flow

1. **System Detection**: Frontend calls Tauri commands → System service uses macOS APIs → Returns monitor/window/app data
2. **Profile Creation**: User captures current layout → Data saved to SQLite → Profile stored with relationships
3. **Profile Activation**: User selects profile → Services restore monitor positions → Launch apps → Open browser tabs
4. **Automation**: Rules evaluated on triggers → Automatic profile switching

## 🔒 Security

- Input validation on all API endpoints
- SQL injection prevention via prepared statements
- Request authentication and authorization
- Data encryption for sensitive information

## 🧪 Testing

- Unit tests for all services and repositories
- Integration tests for API endpoints
- System detection tests with mock data
- End-to-end tests for critical user flows

## 🚀 Deployment

Built for macOS with Tauri bundler. Supports:
- macOS .app bundles
- Auto-updates via Tauri
- Code signing for distribution</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/README.md