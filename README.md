# iggy-sample

A sample Rust project demonstrating message streaming with [Iggy](https://iggy.rs) — a persistent, high-performance message streaming platform.

## Overview

The project contains two binaries that communicate through an Iggy server:

- **`producer`** — creates a stream and topic (if they don't exist), then continuously sends batches of 10 messages every 500ms.
- **`consumer`** — polls the stream for messages every 500ms, printing each message's offset and payload. Stops automatically after 10 consecutive empty batches.

Both binaries use the stream `sample-stream`, topic `sample-topic`, and partition `1`.

## Technologies

| Dependency | Version | Purpose |
|---|---|---|
| [Iggy](https://iggy.rs) | 0.8.0 | Message streaming platform |
| [Tokio](https://tokio.rs) | 1.48.0 | Async runtime |
| [Tracing](https://docs.rs/tracing) | 0.1.43 | Structured logging |
| [Tracing Subscriber](https://docs.rs/tracing-subscriber) | 0.3.22 | Log output formatting |

## Prerequisites

- [Rust](https://rustup.rs/) (2024 edition or later)
- A running Iggy server (see [Iggy quickstart](https://docs.iggy.rs/introduction/quick-start/))

## Running

Start the Iggy server first, then run each binary in a separate terminal.

**Producer** — sends messages continuously:
```sh
cargo run --bin producer
```

**Consumer** — reads messages until 10 empty batches are received:
```sh
cargo run --bin consumer
```

## Building

```sh
cargo build --release
```

Binaries will be available at `target/release/producer` and `target/release/consumer`.

## Testing

Run the producer and consumer together to verify end-to-end message flow:

1. Start the Iggy server.
2. In one terminal, start the producer:
   ```sh
   cargo run --bin producer
   ```
3. In another terminal, start the consumer:
   ```sh
   cargo run --bin consumer
   ```

The consumer should log each received message with its offset and payload (e.g., `message-1`, `message-2`, ...).

To run Rust's built-in checks:
```sh
cargo check
cargo clippy
```
