# Smoothie - Desktop Workspace Profile Manager

[![CI/CD](https://github.com/SinanGncgl/smoothie/actions/workflows/ci.yml/badge.svg)](https://github.com/SinanGncgl/smoothie/actions/workflows/ci.yml)
[![Security](https://github.com/SinanGncgl/smoothie/actions/workflows/security.yml/badge.svg)](https://github.com/SinanGncgl/smoothie/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Contributors](https://img.shields.io/github/contributors/SinanGncgl/smoothie)](https://github.com/SinanGncgl/smoothie/graphs/contributors)
[![Issues](https://img.shields.io/github/issues/SinanGncgl/smoothie)](https://github.com/SinanGncgl/smoothie/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/SinanGncgl/smoothie/blob/main/CONTRIBUTING.md)

A Tauri-based desktop application for managing and switching between different workspace profiles on macOS. Capture monitor arrangements, window positions, and running applications to create reusable workspace configurations.

## ✨ Features

- **System Detection**: Capture current monitor setup, window positions, and running apps
- **Profile Management**: Save and restore workspace configurations
- **Monitor Layout Application**: Apply monitor arrangements to macOS display settings
- **Automation Rules**: Trigger profile switches based on conditions
- **Modern UI**: Clean, responsive interface built with Radix UI

## 📸 Screenshots

### Dashboard
![Dashboard](screenshots/dashboard.png)
*Main dashboard showing workspace profiles and system status*

### Profile Management
![Profile Management](screenshots/profiles.png)
*Create and manage workspace profiles with monitor layouts and app configurations*

### Monitor Editor
![Monitor Editor](screenshots/monitor-editor.png)
*Visual monitor layout editor with drag-and-drop positioning and system application*

### System Capture
![System Capture](screenshots/capture.png)
*Capture current monitor setup and running applications*

### Settings
![Settings](screenshots/settings.png)
*Configure app preferences and automation rules*

### Help & Support
![Help & Support](screenshots/help.png)
*FAQ, feedback submission, and support resources*

## 🚀 Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd smoothie

# Install dependencies
pnpm install

# Start development environment
pnpm run dev
```

## 🔧 Setup

### Prerequisites
- **Node.js 18+** - [Download](https://nodejs.org/)
- **Rust 1.70+** - [Install](https://rustup.rs/)
- **Tauri CLI** - Install with `cargo install tauri-cli`
- **macOS** (required for Tauri development)
- **Docker & Docker Compose** (for database)
- **displayplacer** (for monitor layout management)

### Installing Tauri CLI

```bash
cargo install tauri-cli
```

### Installing displayplacer

```bash
# Using Homebrew
brew install jakehilborn/jakehilborn/displayplacer

# Or download from GitHub releases
# https://github.com/jakehilborn/displayplacer/releases
```

**Note**: Monitor layout changes require administrator privileges. The app will attempt to apply changes automatically, but may fall back to providing a manual command.

### Database Setup

The app uses PostgreSQL running in Docker:

```bash
# Start PostgreSQL database
docker-compose up -d postgres

# The database will be available at localhost:5432
```

## 📚 Documentation

For comprehensive documentation on the codebase architecture, data flows, and development guides:

### 📖 [Complete Documentation Guide](docs/README.md)

This includes:
- **Architecture Overview** - How the Tauri + Next.js + Rust stack works
- **Data Flow Diagrams** - System detection and profile management flows
- **File-by-File Documentation** - Detailed explanations of every major component
- **Development Workflow** - Step-by-step guides for adding features
- **Navigation Guide** - Essential files for new developers
- **Troubleshooting** - Common issues and debug tools

### 🏗️ Tech Stack

- **Frontend**: Next.js 16 + React 19 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri 2.0 + PostgreSQL
- **UI**: Radix UI + shadcn/ui components
- **Platform**: macOS (CoreGraphics APIs)

## 🔧 Development



### Development Scripts
- `pnpm run dev` - Start full development stack (Tauri + Next.js)
- `pnpm run build` - Build for production
- `cargo build` - Build Rust backend only
- `cargo test` - Run Rust tests

### Docker Development

For containerized development:

```bash
# Start full development environment
docker-compose up

# Or run specific services
docker-compose up frontend    # Next.js dev server on port 3000
docker-compose up postgres    # PostgreSQL database

# Rebuild containers when dependencies change
docker-compose up --build frontend
```

This sets up:
- **Frontend**: Next.js development server with hot reload on port 3000
- **Database**: PostgreSQL with persistent volume
- **Networking**: Isolated network for service communication

**Note**: If you modify `package.json` or `pnpm-lock.yaml`, rebuild the frontend container with `docker-compose up --build frontend` to install updated dependencies.

## 🚀 Production Deployment

### Building for Production

```bash
# Build the application
pnpm run tauri build

# The built app will be in src-tauri/target/release/bundle/
```

### Environment Variables

For production, set the database URL:

```bash
DATABASE_URL=postgresql://username:password@localhost:5432/smoothie_prod
```

The application will automatically run database migrations on startup.

## 📁 Project Structure

```
├── app/                 # Next.js pages
├── components/          # React components
├── hooks/              # Custom React hooks
├── lib/                # Utilities and API wrappers
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── handlers/   # Tauri command handlers
│   │   ├── services/   # Business logic
│   │   ├── repositories/ # Data access
│   │   └── models/     # Data structures
│   └── Cargo.toml
├── docs/               # 📖 Complete documentation
└── package.json
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details on:

- Development setup
- Coding standards
- Pull request process
- Testing guidelines

### Quick Start for Contributors

```bash
git clone https://github.com/SinanGncgl/smoothie.git
cd smoothie
pnpm install
docker-compose up -d postgres
pnpm tauri dev
```

## 📋 Community

- [📖 Documentation](docs/README.md)
- [🐛 Issue Tracker](https://github.com/SinanGncgl/smoothie/issues)
- [💬 Discussions](https://github.com/SinanGncgl/smoothie/discussions)
- [🤝 Code of Conduct](CODE_OF_CONDUCT.md)

## 📞 Support

If you have any questions or need help:

- 📧 Open an [issue](https://github.com/SinanGncgl/smoothie/issues) on GitHub
- 💬 Check the [documentation](docs/README.md)
- 🤝 Read our [contributing guide](CONTRIBUTING.md)

---
