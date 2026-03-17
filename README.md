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
- Automatic OpenAPI 3.0 specification generation
- Built-in Swagger UI
- Type-safe request/response schemas via `schemars`

## Quick Start

### Basic usage

```rust
// main.rs
use fastrust::{APIApp, APIRouter, RouteConfig};
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
    api.get("/hello/{name}", hello, RouteConfig::default());

    // Create another router with prefix /v1
    let mut v1 = APIRouter::new("/v1");
    v1.get("/", root, RouteConfig::default());

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
use fastrust::{APIApp, APIRouter, RouteConfig};
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
       "Cannot acquire counter\n".to_string()
    }
}

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

#[tokio::main]
async fn main() {
    let mut root = APIRouter::new("/");
    root.get("/increment/{n}", increment, RouteConfig::default());

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

### With OpenAPI documentation

fastrust automatically generates OpenAPI specifications and serves a Swagger UI:

```rust
use fastrust::{APIApp, APIRouter, RouteConfig};
use axum::extract::{Json, Path};
use schemars::{JsonSchema, Serialize};

#[derive(JsonSchema, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(JsonSchema, Serialize)]
struct CreateUser {
    name: String,
}

async fn get_user(Path(id): Path<i32>) -> Json<User> {
    Json(User { id, name: "Alice".to_string() })
}

async fn create_user(Json(body): Json<CreateUser>) -> Json<User> {
    Json(User { id: 1, name: body.name })
}

#[tokio::main]
async fn main() {
    let mut api = APIRouter::new("/api");
    
    api.get(
        "/user/{id}",
        get_user,
        RouteConfig::default()
            .summary("Get a user by ID")
            .response::<User>(200, "User found")
            .empty_response(404, "User not found")
    );
    
    api.post(
        "/user",
        create_user,
        RouteConfig::default()
            .summary("Create a new user")
            .response::<User>(200, "User created")
    );

    APIApp::new()
        .set_title("My API")
        .set_version("1.0.0")
        .set_host("localhost")
        .set_port(8080)
        // OpenAPI spec available at /openapi.json (default)
        // Swagger UI available at /docs (default)
        .register_router(api)
        .run().await;
}
```

Visit `http://localhost:8080/docs` to interact with your API documentation.

### Customizing OpenAPI and docs paths

```rust
APIApp::new()
    .set_title("My API")
    .set_openapi_path("/api-spec.json")  // Custom OpenAPI JSON path
    .set_docs_path("/swagger")           // Custom Swagger UI path
    .register_router(api)
    .run().await;
```

## Configuration

| Method | Default | Description |
|--------|---------|-------------|
| `set_title()` | `"Axum API"` | API title |
| `set_description()` | `None` | API description |
| `set_version()` | `"0.0.1"` | API version |
| `set_host()` | `None` | Server host (e.g., `"localhost"`) |
| `set_port()` | `6969` | Server port |
| `set_openapi_path()` | `"/openapi.json"` | OpenAPI JSON endpoint |
| `set_docs_path()` | `"/docs"` | Swagger UI endpoint |

## TODOs

- [ ] Request validation (e.g., `#[validate]` attributes)
- [ ] More extractor types (Form, Multipart)
- [ ] Built-in middleware (CORS, authentication)
- [ ] Response compression
- [ ] WebSocket support

## Releases

### 0.3.0

**Major refactoring and OpenAPI improvements:**

- Automatic Swagger UI - Interactive API documentation at `/docs`
- OpenAPI 3.0 specification - Auto-generated at `/openapi.json`
- Restructured modules - Better organization for maintainability:

### 0.2.0

- Add app state support

### 0.1.0

- Initial release

## License

MIT
