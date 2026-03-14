use fastrust::{APIApp, APIRouter};
use axum::extract::State;

use std::sync::{Arc, Mutex};


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
    api.get("/", increment);

    let state = AppState {
        counter: Arc::new(Mutex::new(0))
    };

    APIApp::new_with_state(state)
        .set_title("fastrust app") 
        .register_router(api)
        .run().await;
}


fn generate(method: String, path: String, ) {

}
