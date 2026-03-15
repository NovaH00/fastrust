# fastrust

A FastAPI-inspired web framework for building APIs quickly in Rust.

> ⚠️ **WIP** - This library is currently under active development.

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

### Basic usage
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
    v1.include_router(api);

    APIApp::new()
        .set_title("fastrust app")
        .set_host("0.0.0.0")
        .set_port(6969)
        .register_router(v1)
        .run().await;
}
```

### With app state
```rust
// main.rs
use fastrust::{APIApp, APIRouter};
use axum::extract::{State, Path};
use std::sync::{Arc, Mutex};

async fn increment(
    Path(n): Path<i32>,
    State(s): State<AppState>
) -> String {
    if let Ok(mut counter) = s.counter.lock() {
        *counter += n;
        format!("Counter incremented by {n}. Current value {}\n", *counter)

    } else {
       "Cannot aquire counter\n".to_string() 
    }   
}

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

#[tokio::main]
async fn main() {
    let mut root = APIRouter::new("/");
    root.get("/increment/{n}", increment);

    let state = AppState {
        counter: Arc::new(Mutex::new(0))
    };

    APIApp::new_with_state(state)
        .set_title("fastrust app") 
        .register_router(root)
        .run().await;
}
```
```bash
$ curl localhost:6969/increment/32
Counter incremented by 32. Current value 32

$ curl localhost:6969/increment/69
Counter incremented by 69. Current value 101

$ curl localhost:6969/increment/20
Counter incremented by 20. Current value 121
```

## TODOs
- [] Add documents generation (openapi.json, swagger UI,...)

## Releases
- 0.1.0: Initial release
- 0.2.0: Add app state support

## License
MIT
