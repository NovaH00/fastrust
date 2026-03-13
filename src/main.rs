use fastrust::{APIApp, APIRouter};
use axum::{
    extract::Path
};

fn init_logging() {
    tracing_subscriber::fmt().init();
}

async fn root() -> &'static str {
    "Hello from fastrust!\n"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}\n", name)
}

#[tokio::main]
async fn main() {
    init_logging();

    // Create an router with the prefix /api
    let mut api = APIRouter::new("/api");
    // Add an handler to the endpoint /api/hello/{name} with the method GET 
    api.get("/hello/{name}", hello); 

    // Create an router with the prefix /v1
    let mut v1 = APIRouter::new("/v1");
    // And handler to the enpoint /v1 with the method GET
    v1.get("/", root); // /v1
    
    // Combine the api router to the v1 router
    // Now all the api router endpoints must start
    // with the v1 router prefix (e.g. /v1/api/hello/{name})
    v1.include_router(&api);  

    APIApp::new()
        .set_title("fastrust app") 
        .set_host("0.0.0.0")
        .set_port(6969)
        .register_router(v1) 
        .run().await;
}
