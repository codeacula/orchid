#!/usr/bin/env bash

# =============================================================================
# check-env.sh — Verify required development tools are installed
# Frontend tooling: bun is used instead of node/npm for this project.
# (node and npm are listed here for reference but bun is the required tool.)
# =============================================================================

set -e

TOOLS=(
    "rustc"
    "cargo"
    "bun"
    "docker"
    "just"
    "sqlx"
)

echo "Checking for required development tools..."
echo

MISSING=0

for tool in "${TOOLS[@]}"; do
    if command -v "$tool" &> /dev/null; then
        VERSION=$($tool --version 2>&1 | head -n 1)
        printf "✓ %-10s %s\n" "$tool:" "$VERSION"
    else
        printf "✗ %-10s NOT INSTALLED\n" "$tool:"
        MISSING=$((MISSING + 1))
    fi
done

echo

if [ $MISSING -eq 0 ]; then
    echo "All required tools are installed! ✓"
    exit 0
else
    echo "Missing $MISSING tool(s). Please install them before proceeding."
    exit 1
fi
