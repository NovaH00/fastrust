# fastrust

A FastAPI-inspired web framework for building APIs quickly in Rust.

> ⚠️ **Work in Progress** - This library is currently under active development.

## Installation
```bash
cargo add fastrust
```

## Features

- FastAPI-inspired API design
- Simple and intuitive router system
- Built on top of [axum](https://github.com/tokio-rs/axum)
- Async/await support

## Quick Start

```rust
// main.rs
use fastrust::{APIApp, APIRouter};
use axum::extract::Path;

async fn root() -> &'static str {
    "Hello from fastrust!\n"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}\n", name)
}

#[tokio::main]
async fn main() {
    // Create a router with prefix /api
    let mut api = APIRouter::new("/api");
    api.get("/hello/{name}", hello);

    // Create another router with prefix /v1
    let mut v1 = APIRouter::new("/v1");
    v1.get("/", root);

    // Combine routers - endpoints become /v1/api/hello/{name}
    v1.include_router(&api);

    APIApp::new()
        .set_title("fastrust app")
        .set_host("0.0.0.0")
        .set_port(6969)
        .register_router(v1)
        .run().await;
}
```

```bash
$ cargo run
Registering paths:
	GET /v1
	GET /v1/api/hello/{name}
Server is listening on 0.0.0.0:6969
```

## License

MIT
