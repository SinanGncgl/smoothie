#!/bin/bash

# Database setup script for Smoothie
# This script helps set up PostgreSQL database for development

set -e

echo "Smoothie Database Setup"
echo "======================="

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "Error: PostgreSQL is not installed. Please install PostgreSQL first."
    echo "Visit: https://www.postgresql.org/download/"
    exit 1
fi

# Check if PostgreSQL is running
if ! pg_isready -q; then
    echo "Error: PostgreSQL is not running. Please start PostgreSQL service."
    exit 1
fi

# Database configuration
DB_NAME="smoothie_dev"
DB_USER="smoothie_user"
DB_PASSWORD="sinan1g1"

echo "Setting up database: $DB_NAME"
echo "User: $DB_USER"

# Create user and database
psql -U $(whoami) -d postgres << EOF
-- Create user if it doesn't exist
DO \$\$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = '$DB_USER') THEN
      CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';
   END IF;
END
\$\$;

-- Create database if it doesn't exist
SELECT 'User $DB_USER created or already exists' as status;
EOF

# Create database separately (simpler approach)
psql -U $(whoami) -d postgres -c "CREATE DATABASE $DB_NAME OWNER $DB_USER;" 2>/dev/null || echo "Database $DB_NAME already exists"

echo "Database $DB_NAME created or already exists"

echo ""
echo "Database setup complete!"
echo ""
echo "Update your .env file with:"
echo "DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME"
echo ""
echo "To run migrations manually (optional):"
echo "cargo install sqlx-cli"
echo "sqlx migrate run --database-url \"postgresql://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME\""