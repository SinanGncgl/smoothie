# Contributing to Smoothie

Thank you for your interest in contributing to Smoothie! 🎉 We welcome contributions from everyone, whether you're fixing bugs, adding features, improving documentation, or helping with testing.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Documentation](#documentation)
- [Issue Reporting](#issue-reporting)
- [Community](#community)

## 🤝 Code of Conduct

This project follows a [Code of Conduct](CODE_OF_CONDUCT.md) to ensure a welcoming environment for all contributors. By participating, you agree to uphold this code.

## 🚀 How to Contribute

### Types of Contributions

- **🐛 Bug Fixes**: Fix existing issues
- **✨ Features**: Add new functionality
- **📚 Documentation**: Improve docs, tutorials, or examples
- **🧪 Testing**: Add or improve tests
- **🎨 UI/UX**: Improve user interface and experience
- **🔧 Maintenance**: Code refactoring, dependency updates, CI/CD improvements

### Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Create** a feature branch from `main`
4. **Make** your changes
5. **Test** your changes thoroughly
6. **Commit** with clear, descriptive messages
7. **Push** to your fork
8. **Create** a Pull Request

## 🛠️ Development Setup

### Prerequisites

- **Node.js 18+** - [Download here](https://nodejs.org/)
- **Rust 1.70+** - [Install here](https://rustup.rs/)
- **pnpm** - `npm install -g pnpm`
- **Tauri CLI** - `cargo install tauri-cli`
- **Docker & Docker Compose** - For database
- **macOS** - Required for Tauri development

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/your-username/smoothie.git
cd smoothie

# Install dependencies
pnpm install

# Start PostgreSQL database
docker-compose up -d postgres

# Start development
pnpm tauri dev
```

### Detailed Setup

See the [README.md](README.md) for comprehensive setup instructions.

## 🏗️ Project Structure

```
smoothie/
├── app/                    # Next.js pages (frontend)
├── components/            # React components
├── contexts/              # React contexts
├── hooks/                 # Custom React hooks
├── lib/                   # Utilities and API wrappers
├── public/                # Static assets
├── src-tauri/            # Rust backend (Tauri)
│   ├── src/
│   │   ├── handlers/     # Tauri command handlers
│   │   ├── repositories/ # Data access layer
│   │   ├── services/     # Business logic
│   │   └── models/       # Data structures
│   └── Cargo.toml
├── docs/                  # Documentation
├── screenshots/           # App screenshots
└── .github/              # GitHub configuration
```

## 💻 Coding Standards

### Frontend (TypeScript/React)

- **Language**: TypeScript (strict mode enabled)
- **Framework**: Next.js 16 with App Router
- **Styling**: Tailwind CSS with shadcn/ui components
- **State Management**: React Context + hooks
- **Linting**: ESLint with Next.js config
- **Formatting**: Prettier (via ESLint)

```typescript
// ✅ Good: Clear types, descriptive names
interface UserProfile {
  id: string
  name: string
  email: string
  preferences: UserPreferences
}

// ❌ Bad: Any types, unclear naming
interface User {
  id: any
  n: string
  e: string
  p: any
}
```

### Backend (Rust)

- **Edition**: 2021
- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy` (warnings as errors)
- **Testing**: Unit tests with meaningful assertions
- **Error Handling**: Custom error types with `thiserror`

```rust
// ✅ Good: Clear error handling, descriptive names
#[derive(Debug, thiserror::Error)]
pub enum SmoothieError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

// ❌ Bad: Generic errors, unclear naming
pub fn process_data(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ...
}
```

### General Guidelines

- **Commits**: Use conventional commits (`feat:`, `fix:`, `docs:`, etc.)
- **Branch Names**: `feature/description`, `fix/issue-number`, `docs/update`
- **PR Titles**: Clear, descriptive titles under 50 characters
- **PR Descriptions**: Explain what, why, and how

## 🧪 Testing

### Frontend Testing

```bash
# Run type checking
pnpm run type-check

# Run linting
pnpm run lint

# Build to check for errors
pnpm run build
```

### Backend Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run clippy (linting)
cargo clippy -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

### Integration Testing

- Test database operations with the provided Docker setup
- Verify Tauri commands work end-to-end
- Test UI interactions in the built application

## 🔄 Pull Request Process

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/issue-number-description
   ```

2. **Make Changes**
   - Follow coding standards
   - Add tests for new features
   - Update documentation
   - Ensure CI passes

3. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add new feature description"
   ```

4. **Push and Create PR**
   - Push to your fork
   - Create PR with clear description
   - Reference related issues

5. **PR Review**
   - Address review feedback
   - Keep PR focused on single feature/fix
   - Squash commits if requested

### PR Checklist

- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Breaking changes documented
- [ ] PR description is clear
- [ ] Related issues linked

## 📚 Documentation

### Code Documentation

- **Rust**: Use `///` doc comments for public APIs
- **TypeScript**: Use JSDoc comments for complex functions
- **Components**: Document props and behavior

### User Documentation

- Update README for new features
- Add screenshots for UI changes
- Update setup instructions if needed

## 🐛 Issue Reporting

### Bug Reports

- Use the bug report template
- Include steps to reproduce
- Provide system information
- Attach logs/screenshots

### Feature Requests

- Use the feature request template
- Explain the problem it solves
- Describe the proposed solution
- Consider alternative approaches

## 🌟 Community

### Getting Help

- **Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Documentation**: Check docs first

### Recognition

Contributors are recognized in:
- GitHub contributor stats
- Release notes
- Future acknowledgments

### Communication

- Be respectful and constructive
- Use clear, descriptive language
- Provide context for your suggestions
- Be open to feedback

## 🙏 Thank You

Your contributions make Smoothie better for everyone! Whether it's a small bug fix or a major feature, every contribution is valued and appreciated.

Happy coding! 🚀