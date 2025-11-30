# Database Setup Guide

This guide will help you set up PostgreSQL for the Smoothie application.

## Prerequisites

- PostgreSQL 12+ installed on your system
- Rust and Cargo installed

## PostgreSQL Installation

### macOS (using Homebrew) - RECOMMENDED
```bash
brew install postgresql@14
brew services start postgresql@14

# Verify installation
psql -U $(whoami) -d postgres -c "SELECT version();"
```

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### Windows
Download and install PostgreSQL from: https://www.postgresql.org/download/windows/

## Database Setup

1. **Install PostgreSQL** (see installation section above)

2. **Run the setup script**:
```bash
cd src-tauri
./setup_db.sh
```

3. **Update your `.env` file** (the script will provide the exact DATABASE_URL to use)

## Verification

Test your database connection:
```bash
psql -U smoothie_user -d smoothie_dev -c "SELECT 1 as test;"
```

Expected output:
```
 test
------
    1
(1 row)
```

## Running Migrations

The application will automatically run database migrations when it starts. The migrations create all necessary tables and indexes.

## Development Database

For development, you might want to create a separate database:
```bash
sudo -u postgres psql
CREATE DATABASE smoothie_dev OWNER smoothie_user;
GRANT ALL PRIVILEGES ON DATABASE smoothie_dev TO smoothie_user;
\q
```

Then update your `.env`:
```
DATABASE_URL=postgresql://smoothie_user:your_secure_password@localhost:5432/smoothie_dev
```

## Troubleshooting

### Connection Issues
- Ensure PostgreSQL is running: `brew services list` (macOS) or `sudo systemctl status postgresql` (Linux)
- Check if the database exists: `psql -U smoothie_user -d smoothie -h localhost`
- Verify the DATABASE_URL format

### Permission Issues
- Make sure the database user has proper permissions
- Check PostgreSQL logs for detailed error messages

### Migration Issues
- If migrations fail, you can manually run them using sqlx CLI:
```bash
cargo install sqlx-cli
sqlx migrate run --database-url "your_database_url"
```