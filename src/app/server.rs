use axum::{Router, middleware};
use tower_http::normalize_path::NormalizePathLayer;
use crate::APIRouter;
use crate::middleware::log_request;
use crate::RouteConfig;
use std::net::SocketAddr;
use tracing::info;
use super::builder::APIApp;

impl<S> APIApp<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Runs the HTTP server and starts listening for requests.
    ///
    /// This is the main entry point for starting the fastrust application.
    /// It performs the following steps:
    ///
    /// 1. Initializes the tracing subscriber for logging
    /// 2. Generates the OpenAPI specification
    /// 3. Registers all user-defined routes
    /// 4. Registers internal routes (OpenAPI JSON and Swagger UI)
    /// 5. Applies middleware (logging, path normalization)
    /// 6. Binds to the configured host and port
    /// 7. Starts serving requests
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - The host:port address is invalid
    /// - The server fails to bind to the address
    /// - The server encounters a fatal error while running
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fastrust::{APIApp, APIRouter, RouteConfig};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut api = APIRouter::new("/api");
    ///     // Add routes...
    ///
    ///     APIApp::new()
    ///         .set_title("My API")
    ///         .set_port(8080)
    ///         .register_router(api)
    ///         .run().await;
    /// }
    /// ```
    ///
    /// # Default Configuration
    ///
    /// - Host: `127.0.0.1`
    /// - Port: `6969`
    /// - OpenAPI path: `/openapi.json`
    /// - Docs path: `/docs`
    pub async fn run(self) {
        tracing_subscriber::fmt().with_target(false).init();

        let openapi_json = self.generate_openapi_str();
        let openapi_path = self.openapi_path.clone();
        let docs_path = self.docs_path.clone();
        let swagger_html = Self::swagger_html(&openapi_path);

        let host = self.host.clone().unwrap_or_else(|| "127.0.0.1".to_owned());
        let port = self.port.unwrap_or(6969);
        let addr: SocketAddr = format!("{}:{}", host, port).parse().expect("Invalid address");

        let openapi_handler = move |axum::extract::State(_): axum::extract::State<S>| async move {
            (
                [(axum::http::header::CONTENT_TYPE, "application/json")],
                openapi_json
            )
        };

        let docs_handler = move |axum::extract::State(_): axum::extract::State<S>| async move {
            (
                [(axum::http::header::CONTENT_TYPE, "text/html")],
                swagger_html
            )
        };

        let mut axum_router: Router<S> = Router::new();

        for api_router in self.routers {
            for route in api_router.routes {
                axum_router = axum_router.route(&route.path, route.handler);
                info!("Registering: {} {}", route.method, route.path);
            }
        }

        let mut internal_router = APIRouter::new("");
        internal_router.get(
            &openapi_path,
            openapi_handler,
            RouteConfig::default().summary("OpenAPI JSON Specification")
        );
        internal_router.get(
            &docs_path,
            docs_handler,
            RouteConfig::default().summary("Swagger UI Documentation")
        );

        for route in internal_router.routes {
            axum_router = axum_router.route(&route.path, route.handler);
        }

        let axum_router = axum_router
            .layer(middleware::from_fn(log_request))
            .layer(NormalizePathLayer::trim_trailing_slash())
            .with_state(self.state);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        info!("Server listening on http://{}", addr);
        info!("Swagger UI available at http://{}{}", addr, docs_path);

        axum::serve(listener, axum_router).await.unwrap();
    }
}
