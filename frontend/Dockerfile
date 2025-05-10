# Stage 1: Build the React app
FROM node:18 AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install
COPY . .
# Add SKIP_PREFLIGHT_CHECK to bypass ESLint version check
RUN echo "SKIP_PREFLIGHT_CHECK=true" > .env
RUN npm run build

# Stage 2: Serve with nginx
FROM nginx:alpine
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
