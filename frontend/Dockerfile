# =============================================================================
# Frontend Dockerfile — Vue 3 / Vite with Bun
# Multi-stage build: builder → runtime
# =============================================================================

FROM oven/bun:1 as builder

WORKDIR /app

# Copy package files
COPY package.json bun.lock* ./

# Install dependencies with bun (frozen lockfile prevents updates)
RUN bun install --frozen-lockfile

# Copy source code
COPY . .

# Build the Vue application with Vite
RUN bun run build


# Stage 2: Runtime — Serve built app with nginx
FROM nginx:alpine

# Copy the built application from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration (if exists, otherwise use default)
COPY nginx.conf /etc/nginx/nginx.conf 2>/dev/null || echo "Using default nginx config"

# Expose port 80
EXPOSE 80

# Health check
HEALTHCHECK --interval=10s --timeout=5s --retries=3 \
    CMD wget -q -O /dev/null http://localhost/health || exit 1

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
