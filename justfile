# Orchid MVP — Just Recipes
# See: https://just.systems/man/en/

set shell := ["bash", "-c"]

# Default recipe
default:
    @just --list

# Development: Start all services
dev:
    #!/usr/bin/env bash
    set -e
    echo "Starting development environment..."
    docker compose -f docker/docker-compose.yml up --build

# Build: Compile backend and frontend
build:
    #!/usr/bin/env bash
    set -e
    echo "Building backend..."
    cd backend && cargo build --release
    echo "Building frontend..."
    cd ../frontend && bun run build

# Test: Run all tests
test:
    #!/usr/bin/env bash
    set -e
    echo "Running backend tests..."
    cd backend && cargo test
    echo "Running frontend tests..."
    cd ../frontend && bun run test

# Check: Verify code without running tests
check:
    #!/usr/bin/env bash
    set -e
    echo "Checking backend..."
    cd backend && cargo check
    echo "Checking frontend..."
    cd ../frontend && bun run check

# Format: Apply code formatting
fmt:
    #!/usr/bin/env bash
    set -e
    echo "Formatting backend..."
    cd backend && cargo fmt
    echo "Formatting frontend..."
    cd ../frontend && bun run format

# Lint: Check code for issues
lint:
    #!/usr/bin/env bash
    set -e
    echo "Linting backend..."
    cd backend && cargo clippy -- -D warnings
    echo "Linting frontend..."
    cd ../frontend && bun run lint

# Database: Run pending migrations
db-migrate:
    #!/usr/bin/env bash
    set -e
    echo "Running database migrations..."
    cd backend && sqlx migrate run

# Database: Reset (drop and recreate)
db-reset:
    #!/usr/bin/env bash
    set -e
    echo "Resetting database..."
    cd backend && sqlx database drop --yes
    sqlx database create
    sqlx migrate run
    echo "Database reset complete"

# Seed: Populate database with initial data
seed:
    #!/usr/bin/env bash
    set -e
    echo "Seeding database..."
    bash scripts/seed.sh
