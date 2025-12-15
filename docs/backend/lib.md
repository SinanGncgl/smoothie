# src/lib.rs - Library Module Declarations

## Overview
The library entry point that declares all public modules for the Smoothie Rust backend. This file serves as the module index, making all submodules available to other parts of the application.

## Module Structure
```
src/
├── lib.rs (this file)
├── db/ - Database connection and queries
├── error/ - Error handling types
├── handlers/ - Tauri command handlers (API endpoints)
├── logging/ - Structured logging and metrics
├── models/ - Data structures (DTOs and entities)
├── repositories/ - Database access layer
├── security/ - Authentication and middleware
├── services/ - Business logic layer
├── state/ - Application state management
└── utils/ - Utility functions
```

## Purpose
- **Module Organization**: Provides clear separation of concerns
- **Public API**: Exposes modules for use by `main.rs` and tests
- **Dependency Management**: Ensures proper module initialization order
- **Code Navigation**: Serves as a roadmap for the codebase structure

## Module Descriptions

### `db`
Database connection management, migrations, and optimized queries using SQLx.

### `error`
Custom error types (`SmoothieError`) and error handling utilities.

### `handlers`
Tauri command handlers that expose backend functionality to the frontend via IPC (Inter-Process Communication).

### `logging`
Structured logging with tracing, application metrics collection, and request tracing.

### `models`
Data structures including:
- `dto.rs` - API Data Transfer Objects for frontend communication
- `entities.rs` - Database entities matching PostgreSQL schema

### `repositories`
Data access layer providing CRUD operations for all major entities (profiles, monitors, apps, etc.).

### `security`
Authentication, authorization, and request validation middleware.

### `services`
Business logic layer containing the core application logic and orchestrating operations between repositories and external systems.

### `state`
Application state management using thread-safe data structures for caching and shared state.

### `utils`
Utility functions for encryption, validation, and common operations.

## Usage
Modules are imported in `main.rs`:
```rust
mod db;
mod error;
// ... etc
```

And used throughout the application:
```rust
use crate::db::Database;
use crate::services::ProfileService;
// ... etc
```

## Testing
All modules include comprehensive unit tests. The `lib.rs` structure enables:
- Unit testing of individual modules
- Integration testing across module boundaries
- Mock implementations for testing

## Architecture Notes
- **Layered Architecture**: Clear separation between handlers (API), services (business logic), and repositories (data access)
- **Dependency Injection**: Services depend on repositories, handlers depend on services
- **Thread Safety**: All shared state uses `Arc` and appropriate locking mechanisms
- **Error Propagation**: Errors bubble up through the layers with proper context</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/backend/lib.md