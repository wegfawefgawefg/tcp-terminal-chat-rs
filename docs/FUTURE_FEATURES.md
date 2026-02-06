# Future Features (Plan)

This is a learning project. The items below are “good next steps” that force real protocol design and more intentional client UX.

## 1) Real Message Protocol (Framing + Versioning)

Goal: stop relying on “whatever bytes arrived” and define messages unambiguously.

- Add a framing format:
  - Length-prefixed frames (e.g., `u32` big-endian length + payload)
  - Or newline-delimited JSON (simple, but beware embedded newlines)
- Add a version field so the client/server can reject incompatible peers.
- Add message types (examples):
  - `hello` (client -> server): capabilities, desired nickname
  - `chat` (client -> server): text body
  - `system` (server -> client): join/leave notifications, errors
  - `ping`/`pong` (keepalive)
- Add robust parsing with clear error handling:
  - Invalid frame length
  - Unknown message type/version
  - Oversized payloads (DoS guard)

## 2) Client Commands

Goal: make the client feel like a real chat app.

Suggested command syntax: slash commands typed into the input box.

- `/nick <name>`
  - Sets or changes nickname.
  - Server should validate and broadcast nickname changes.
- `/me <action>`
  - Sends an emote-style message.
- `/who`
  - List connected users (server maintains user list).
- `/help`
  - Prints available commands locally.
- `/quit`
  - Clean shutdown (close socket, restore terminal state).

Implementation note: once you have a real protocol, commands should map to structured messages, not special-cased text strings.

## 3) Nicknames + User Identity

Goal: stop being anonymous connections.

- Server assigns a stable connection ID.
- Nickname registry:
  - Validate charset/length.
  - Handle collisions (reject or auto-suffix).
- Broadcast join/leave events.

## 4) Timestamps

Goal: make chat history readable.

- Add a timestamp to each displayed line:
  - Prefer server-generated timestamps so all clients agree.
- Choose a format:
  - `HH:MM:SS` for compactness
  - Or ISO-8601 for clarity

## 5) Message Coloring

Goal: improve readability and “presence” in the TUI.

- Color system messages differently from user chat.
- Per-user coloring:
  - Deterministic mapping from user ID/nickname -> color.
  - Ensure good contrast and avoid too-similar colors.
- Optional: highlight mentions (e.g., `@nick`).

## 6) Client UX Upgrades

- Input editing:
  - Left/right arrows, delete word, home/end
  - History (up/down) for sent messages and commands
- Scrollback:
  - Keep more than 10 messages
  - PageUp/PageDown to scroll; “follow tail” mode toggle
- Connection UI:
  - Show connection state (connected/reconnecting/disconnected)

## 7) Server Hardening

- Handle slow clients without blocking broadcast:
  - Per-client write queues, or non-blocking IO
- Limits:
  - Max clients, max message size, rate limiting
- Better error handling:
  - Cleanly remove disconnected clients
  - Avoid panics, log meaningful reasons

## 8) Networking Beyond Localhost

- Bind configurable host/port (CLI args or env vars).
- Optional TLS (educational stretch goal).

