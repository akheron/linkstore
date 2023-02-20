FROM node:18-alpine AS builder

WORKDIR /app
COPY package.json package-lock.json .
RUN npm ci
COPY . .
RUN npm exec -c 'svelte-kit sync' && npm run build

FROM node:18-alpine
WORKDIR /app
COPY package.json package-lock.json .
RUN npm ci --omit=dev
COPY --from=builder /app/build /app/build

EXPOSE 8080
CMD ["node", "build/index.js"]
