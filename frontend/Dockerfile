# =============================================================================
# Frontend Dockerfile — Vue 3 / Vite with Bun
# Multi-stage build: builder → runtime
# =============================================================================

FROM oven/bun:1 AS runtime

WORKDIR /app

# Copy package files
COPY package.json bun.lock* ./

# Install dependencies with bun (frozen lockfile prevents updates)
RUN bun install --frozen-lockfile

# Copy source code
COPY . .

# Build the Vue application with Vite
RUN bun run build

# Expose the preview server port
EXPOSE 5173

# Health check
HEALTHCHECK --interval=10s --timeout=5s --retries=3 \
    CMD bun -e "fetch('http://127.0.0.1:5173').then((response) => process.exit(response.ok ? 0 : 1)).catch(() => process.exit(1))"

# Start the Vite preview server
CMD ["bun", "x", "vite", "preview", "--host", "0.0.0.0", "--port", "5173"]
