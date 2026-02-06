# tcp-terminal-chat-rs

An educational Rust project: a tiny TCP broadcast server plus a terminal UI (TUI) chat client.

This is intentionally minimal and built for learning (threads, sockets, terminal UI event loops). It is not production-ready.

## What It Is

- `server`: binds `127.0.0.1:7878`, accepts TCP connections, and broadcasts whatever bytes it reads from any client to all clients.
- `client`: terminal UI that connects to `127.0.0.1:7878`, lets you type messages, and renders received messages in a scroll box.

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

### Release Mode

```bash
cargo run --bin server --release
cargo run --bin client --release
```

### Short Alias

This repo includes a Cargo alias so you can type:

```bash
cargo b server
cargo b client
```

## Notes / Limitations

- The “protocol” is currently just raw TCP bytes; messages are not framed and may be split/merged depending on OS buffering.
- Localhost only by default (`127.0.0.1`).
- No auth, no encryption, no persistence.

## Roadmap

See `docs/FUTURE_FEATURES.md`.
