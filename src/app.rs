use axum::{Router, middleware};
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use crate::router::APIRouter;
use crate::middleware::log_request;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct APIApp {
    title:        Option<String>,
    summary:      Option<String>,
    description:  Option<String>,
    version:      String,
    openapi_path: String,
    docs_path:    String,

    host:         String,
    port:         i32,

    routers:      Vec<APIRouter>,
}

impl Default for APIApp {
    fn default() -> Self {
        Self {
            title:        None,
            summary:      None,
            description:  None,
            version:      "0.0.1".to_owned(),
            openapi_path: "/openapi.json".to_owned(),
            docs_path:    "/docs".to_owned(),

            host:         "127.0.0.1".to_owned(),
            port:         6969,

            routers: Vec::new(),
        }
    }
}


impl APIApp {
    pub fn new() -> Self {
        Self::default() 
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn set_summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_owned()); 
        self
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned()); 
        self
    }
    
    pub fn set_version(mut self, version: &str) -> Self {
        self.version = version.to_owned();
        self
    }

    pub fn set_openapi_path(mut self, openapi_path: &str) -> Self {
        self.openapi_path = openapi_path.to_owned();
        self
    }

    pub fn set_docs_path(mut self, docs_path: &str) -> Self {
        self.docs_path = docs_path.to_owned();
        self
    }

    pub fn set_host(mut self, host: &str) -> Self {
        self.host = host.to_owned();
        self
    }

    pub fn set_port(mut self, port: i32) -> Self {
        self.port = port;
        self
    }

    pub fn register_router(mut self, router: APIRouter) -> Self {
        self.routers.push(router);
        self     
    }

    pub async fn run(self) {
        let mut router = Router::<()>::new();

        println!("Registering paths:");
        for api_router in self.routers {
            for route in api_router.routes {
                router = router.route(&route.path, route.handler);
                println!("\t{} {}", route.method, route.path);
            }
        }

        let router = router.layer(middleware::from_fn(log_request));
        let app = NormalizePathLayer::trim_trailing_slash().layer(router);

        // Parse socket address
        let addr: SocketAddr = format!("{}:{}", self.host, self.port)
            .parse()
            .unwrap_or_else(|e| {
                eprint!(
                    "ERROR: Failed to parse socket address.\n\
                        host: `{}`\n\
                        port: `{}`\n\
                        error: {}\n",
                    self.host,
                    self.port,
                    e
                );
                std::process::exit(1);
            });

        // Bind TCP listener
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .unwrap_or_else(|e| {
                eprint!(
                    "ERROR: Failed to bind TCP listener.\n\
                        address: `{}`\n\
                        error: {}\n\
                        Possible causes:\n\
                        - Port already in use\n\
                        - Insufficient permissions\n\
                        - Invalid network interface",
                    addr,
                    e
                );
                std::process::exit(1);
            });

        println!("Server is listening on {}:{}", self.host, self.port);

        // Start server
        axum::serve(listener, axum::ServiceExt::<axum::extract::Request>::into_make_service(app))
        .await
        .unwrap_or_else(|e| {
            eprint!(
                "ERROR: Axum server crashed.\n\
                    address: `{}`\n\
                    error: {}",
                addr,
                e
            );
            std::process::exit(1);
        });
    }
}
