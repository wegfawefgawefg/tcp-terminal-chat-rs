# tcp-terminal-chat-rs
A tiny educational Rust project: a TCP broadcast server plus a terminal UI client for simple chat over localhost. Built to learn networking, threads, and TUI event loops; not production-ready.

## Run

Start the server:

```bash
cargo run --bin server
```

In another terminal (run twice for two clients):

```bash
cargo run --bin client
```

Quit the client with `Ctrl+C`, `Esc`, or `q`.
