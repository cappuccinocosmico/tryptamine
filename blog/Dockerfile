# Stage 1: Build the Dioxus static site
FROM rust:1.71 as builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY . .
RUN cargo install dioxus-cli --locked
RUN dioxus build --release --target web -o dist

# Stage 2: Serve with nginx
FROM nginx:1.25-alpine
LABEL maintainer="you@example.com"
RUN rm -rf /usr/share/nginx/html/*
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
