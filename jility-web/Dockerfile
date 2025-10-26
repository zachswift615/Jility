# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source
COPY . .

# Build Next.js app
# Note: NEXT_PUBLIC_* env vars must be set at build time
ARG NEXT_PUBLIC_API_URL=/api
ARG BACKEND_URL=http://backend:3900

ENV NEXT_PUBLIC_API_URL=${NEXT_PUBLIC_API_URL}
ENV BACKEND_URL=${BACKEND_URL}

RUN npm run build

# Runtime stage
FROM node:20-alpine AS runner

WORKDIR /app

# Don't run as root
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

# Copy built files
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3901

ENV PORT=3901
ENV HOSTNAME="0.0.0.0"

CMD ["node", "server.js"]
