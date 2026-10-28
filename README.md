# Rust Multi-Threaded Web Server

A small HTTP server built from scratch using Rust's standard library,
following the concepts in The Rust Programming Language's final project.

## Features

- TCP listener bound to `127.0.0.1:7878`
- Basic HTTP request handling and HTML responses
- `/` success route and `/sleep` slow route
- 404 page for unknown routes
- Four-worker thread pool using `mpsc`, `Arc`, and `Mutex`
- Graceful worker shutdown using `Drop`, termination messages, and `join`
- Unit tests for routing and thread-pool behavior

## Run

```bash
cargo run
```

The server handles two TCP connections, then shuts down gracefully.
Restart it to accept another two connections.

## Routes

- `GET /` — returns the success page
- `GET /sleep` — waits five seconds, then returns the success page
- Other requests — return `404 NOT FOUND`

## Test

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

To test the running server, use another terminal:

```bash
curl -i http://127.0.0.1:7878/
curl -i http://127.0.0.1:7878/missing
```

The server counts TCP connections, not distinct browser pages. Each server
run accepts two connections and then gracefully shuts down its workers.
