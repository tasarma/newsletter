# 📬 Newsletter Service

A production-grade email newsletter API built in Rust, following the patterns from
**Zero To Production In Rust** by Luca Palmieri.

This project goes well beyond "hello world", it covers the full lifecycle of building,
testing, and deploying a real backend service: domain validation, structured logging,
database migrations, integration testing, and containerized deployment.

## Tech Stack

| Layer | Technology |
|---|---|
| Web Framework | Actix-web 4 |
| Database | PostgreSQL + SQLx |
| Async Runtime | Tokio |
| Configuration | config-rs + secrecy |
| Logging | tracing + tracing-bunyan-formatter |
| Validation | validator + unicode-segmentation |
| HTTP Client | reqwest |
| Containerization | Docker |
| CI | GitHub Actions |

## Features

- **Subscriber management** — `POST /subscriptions` endpoint with full input validation
  (unicode-aware, not just ASCII)
- **Email confirmation flow** — token-based subscription confirmation via email
- **Health check endpoint** — `GET /health_check` for zero-downtime deployments
- **Structured JSON logging** — every request traced end-to-end with correlation IDs via
  `tracing-actix-web`
- **Secret management** — sensitive config values (DB password, SMTP credentials) wrapped
  with `secrecy` to prevent accidental logging
- **Environment-based configuration** — layered config for local, test, and production
  environments
- **Database migrations** — managed with `sqlx migrate`, fully version-controlled
- **Docker** — multi-stage build for a lean production image

## Project Structure
```
src/
├── lib.rs           # Library root — all core logic lives here
├── main.rs          # Binary entry point
├── startup.rs       # App factory / server setup
├── configuration.rs # Config loading and structs
├── routes/          # HTTP handlers
├── domain/          # Domain types with invariants enforced at the type level
└── email_client.rs  # Async email delivery via HTTP
└── temeletry.rs     # Telemetry entry point

tests/
└── healt_check.rs   # Black-box integration tests against a real running server
```

## Getting Started

### Prerequisites

- Rust (stable) — [install via rustup](https://rustup.rs/)
- Docker & Docker Compose
- `sqlx-cli` — `cargo install sqlx-cli --no-default-features --features rustls,postgres`

### Run locally
```bash
# Start PostgreSQL
docker compose up -d

# Run migrations
sqlx migrate run

# Start the server
cargo run
```

The server starts on `http://127.0.0.1:8000` by default.

### Run tests
```bash
cargo test
```

Integration tests spin up a real server instance and a dedicated test database per test run.

## Key Engineering Decisions

**Type-driven domain validation** — `SubscriberName` and `SubscriberEmail` are newtype
wrappers that can only be constructed via a fallible `parse()` method. Invalid data can't
reach the database layer by design — the type system enforces it.

**Zero-downtime deploys** — health check endpoint + Docker-based deployment allows rolling
updates without dropping requests.

**Property-based testing** — uses `quickcheck` to generate hundreds of arbitrary inputs
and verify that valid subscriber names are always accepted and invalid ones are always
rejected.

**Separation of concerns** — the app is structured as a library (`lib.rs`) consumed by a
thin binary (`main.rs`). This makes integration testing straightforward — tests import the
library directly without spawning a subprocess.
