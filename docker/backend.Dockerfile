# =============================================================================
# Backend Dockerfile — Rust API with cargo-chef multi-stage build
# 3-stage build: planner → builder → runtime
# =============================================================================

# Stage 1: Planner — Prepare recipe for dependencies
FROM rust:latest AS planner

RUN cargo install cargo-chef

WORKDIR /app/backend

# Copy the backend crate and local patched dependency
COPY backend ./
COPY wiremock-patch ../wiremock-patch

# Create the recipe for dependencies
RUN cargo chef prepare --recipe-path recipe.json


# Stage 2: Builder — Compile dependencies and application
FROM rust:latest AS builder

RUN cargo install cargo-chef

WORKDIR /app/backend

# Copy the recipe from planner stage
COPY --from=planner /app/backend/recipe.json recipe.json

# Copy the local patched dependency needed during dependency resolution
COPY wiremock-patch ../wiremock-patch

# Build dependencies (this layer is cached as long as dependencies don't change)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the backend crate and local patched dependency
COPY backend ./

# Build the application
RUN cargo build --release


# Stage 3: Runtime — Minimal image with just the binary
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    curl \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/backend/target/release/orchid /app/orchid

# Create a non-root user for security
RUN useradd -m -u 1000 orchid && chown -R orchid:orchid /app
USER orchid

# Expose the backend port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=10s --timeout=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["/app/orchid"]
