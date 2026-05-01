# Narya

Cross-platform VPN/Proxy client powered by Tauri v2, Vue 3, and Rust.

## Project Structure

- `apps/desktop`: Tauri + Vue 3 frontend
- `crates/`: Rust workspace crates for core logic
- `resources/`: Sidecar cores and other resources
- `architecture/`: Architecture design documents
- `ui/`: UI design assets

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Bun](https://bun.sh/)
- [Tauri v2 dependencies](https://v2.tauri.app/start/prerequisites/)

### Setup

```bash
bun install
```

### Run Desktop App

```bash
cd apps/desktop
bun run tauri dev
```

### Lint & Test

```bash
# Frontend
cd apps/desktop
bun run lint
bun run typecheck
bun run test

# Rust
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## License

MIT
