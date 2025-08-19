# Builder stage: install deps and build Next.js app
FROM node:20-alpine AS deps
WORKDIR /app

# Install OS deps for sharp/optional native deps
RUN apk add --no-cache libc6-compat

# Install dependencies with a clean, reproducible lockfile
COPY package.json package-lock.json ./
RUN npm ci --include=dev

# Copy rest of the source and build
COPY . .

# Ensure Next builds in production mode
ENV NODE_ENV=production
# Allow customizing API base URL at build-time for SSG assets if needed
ARG API_BASE_URL
ENV API_BASE_URL=${API_BASE_URL}

RUN npm run build

# Runner stage: minimal image to run Next.js server
FROM node:20-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production

# Create non-root user
RUN addgroup -S nextjs && adduser -S nextjs -G nextjs

# Copy only necessary build output and runtime files
# Using standalone output includes the minimal server runtime in .next/standalone
COPY --from=deps /app/public ./public
COPY --from=deps /app/.next/standalone ./
COPY --from=deps /app/.next/static ./.next/static

# Expose Next.js port
EXPOSE 3000

# Runtime env for API base URL; can be overridden by compose/k8s
ENV PORT=3000
ENV HOSTNAME=0.0.0.0
ENV API_BASE_URL=http://api:8080

USER nextjs

CMD ["node", "server.js"]
