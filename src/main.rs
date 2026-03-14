use fastrust::{APIApp, APIRouter};
use axum::{
    extract::{Path, State}
};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

async fn root(Path(name): Path<String>) -> String {
    format!("Hello {name}!\n")
}

async fn increment(State(s): State<AppState>) -> String {
    let mut counter = s.counter.lock().unwrap();
    *counter += 1;

    format!("Counter incremented. Current value {}", *counter)
}

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

#[tokio::main]
async fn main() {

    
    let mut api = APIRouter::new("/");
    api.post("/{name}", root);

    let state = AppState {
        counter: Arc::new(Mutex::new(0))
    };

    APIApp::new()
        .set_title("fastrust app") 
        .register_router(api)
        .run().await;
}
