# 🦀 Rust World Cup API

A small Rust backend built with [axum](https://github.com/tokio-rs/axum). It serves a few JSON endpoints, including 2026 FIFA World Cup data read from [data/world_cup.json](data/world_cup.json).

## Live demo

Deployed on Render: **<https://aise-rust-be1.onrender.com>**

World Cup endpoint: **<https://aise-rust-be1.onrender.com/world-cup>**

## Endpoints

| Method | Path | Description |
| ------ | ---- | ----------- |
| GET | `/` | Service info (name, message, version). |
| GET | `/data` | Sample JSON payload. |
| GET | `/greet?name=Katia` | Greets the given `name` (defaults to `stranger`). |
| GET | `/world-cup` | Serves the 2026 FIFA World Cup data from [data/world_cup.json](data/world_cup.json). |

## Running locally

```sh
cargo run
```

The server binds to `0.0.0.0:$PORT`, defaulting to port `3000` if `PORT` is unset.

```sh
curl http://localhost:3000/world-cup
```

## Tech stack

- [Rust](https://www.rust-lang.org/) (edition 2024)
- [axum](https://github.com/tokio-rs/axum) — web framework
- [tokio](https://tokio.rs/) — async runtime
- [serde_json](https://github.com/serde-rs/json) — JSON serialization
