use axum::{Router, middleware};
use tower_http::normalize_path::NormalizePathLayer;
use crate::APIRouter;
use crate::middleware::log_request;
use std::net::SocketAddr;
use tracing::{info, warn, error};
use openapiv3 as oa;
use indexmap::IndexMap;
use crate::routing::Method;
use crate::RouteConfig;

#[derive(Debug)]
pub struct APIApp<S = ()> 
where 
    S: Clone + Send + Sync + 'static,
{
    title:        Option<String>,
    summary:      Option<String>,
    description:  Option<String>,
    version:      String,
    openapi_path: String,
    docs_path:    String,
    state:        S,

    host:         Option<String>,
    port:         Option<i32>,

    routers:      Vec<APIRouter<S>>,
}

impl Default for APIApp<()> 
{
    fn default() -> Self {
        Self {
            title:        None,
            summary:      None,
            description:  None,
            version:      "0.0.1".to_owned(),
            openapi_path: "/openapi.json".to_owned(),
            docs_path:    "/docs".to_owned(),
            state:        (),
            host:         None,
            port:         None,
            routers:      vec![],
        }
    }
}

impl APIApp<()> {
    pub fn new() -> Self {
        Self::default() 
    }
}

impl<S> APIApp<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new_with_state(state: S) -> Self {
        Self {
            title:        None,
            summary:      None,
            description:  None,
            version:      "0.0.1".to_owned(),
            openapi_path: "/openapi.json".to_owned(),
            docs_path:    "/docs".to_owned(),
            state,
            host:         None,
            port:         None,
            routers:      vec![],
        }
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
        self.host = Some(host.to_owned());
        self
    }

    pub fn set_port(mut self, port: i32) -> Self {
        self.port = Some(port);
        self
    }

    pub fn register_router(mut self, router: APIRouter<S>) -> Self {
        self.routers.push(router);
        self     
    }

    pub fn generate_openapi_str(&self) -> String {
        // 1. Initialize the paths map using IndexMap to match openapiv3 requirements
        let mut paths: IndexMap<String, oa::ReferenceOr<oa::PathItem>> = IndexMap::new();

        // 2. Iterate through all routers and their respective routes
        for router in &self.routers {
            for route in &router.routes {
                // Ensure we have a PathItem for this specific URL
                let path_item_ref = paths
                    .entry(route.path.clone())
                    .or_insert(oa::ReferenceOr::Item(oa::PathItem {
                        get: None,
                        post: None,
                        put: None,
                        delete: None,
                        options: None,
                        head: None,
                        patch: None,
                        trace: None,
                        servers: vec![],
                        parameters: vec![],
                        extensions: IndexMap::new(),
                        summary: None,
                        description: None,
                    }));

                if let oa::ReferenceOr::Item(path_item) = path_item_ref {
                    // 3. Assign the operation to the correct HTTP method slot
                    match &route.method {
                        Method::Get => path_item.get = Some(route.operation.clone()),
                        Method::Post => path_item.post = Some(route.operation.clone()),
                        Method::Put => path_item.put = Some(route.operation.clone()),
                        Method::Delete => path_item.delete = Some(route.operation.clone()),
                        Method::Patch => path_item.patch = Some(route.operation.clone()),
                        Method::Head => path_item.head = Some(route.operation.clone()),
                        Method::Trace => path_item.trace = Some(route.operation.clone()),
                        Method::Options => path_item.options = Some(route.operation.clone()),
                        Method::Connect => ()
                    }
                }
            }
        }

        // 4. Build the Global Info object using App metadata
        let info = oa::Info {
            title: self.title.clone().unwrap_or_else(|| "Axum API".to_string()),
            description: self.description.clone(),
            version: self.version.clone(),
            // You can use the summary field in description if title is used for something else
            contact: None,
            license: None,
            terms_of_service: None,
            extensions: IndexMap::new(),
        };

        // 5. Build Servers list based on host and port if provided
        let mut servers = Vec::new();
        if let (Some(host), Some(port)) = (&self.host, self.port) {
            servers.push(oa::Server {
                url: format!("http://{}:{}", host, port),
                description: Some("Application Server".to_string()),
                variables: None,
                extensions: IndexMap::new(),
            });
        }

        // 6. Final Assembly
        let openapi_spec = oa::OpenAPI {
            openapi: "3.0.3".to_string(),
            info,
            paths: oa::Paths {
                paths,
                extensions: IndexMap::new(),
            },
            servers,
            components: None,
            security: None,
            tags: vec![],
            external_docs: None,
            extensions: IndexMap::new(),
        };

        match serde_json::to_string_pretty(&openapi_spec) {
            Ok(openapi_str) => openapi_str,
            Err(_) => "".to_owned()
        }
    }

    fn swagger_html(openapi_path: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Swagger UI</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
    <style>
        body {{ margin: 0; padding: 0; }}
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {{
            SwaggerUIBundle({{
                url: "{openapi_path}",
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ],
                layout: "BaseLayout",
                deepLinking: true
            }});
        }};
    </script>
</body>
</html>"#
        )
    }

    pub async fn run(self) {
        tracing_subscriber::fmt().with_target(false).init();

        // --- 1. PRE-GENERATE DATA ---
        // Generate the string while we still have access to `self`.
        let openapi_json = self.generate_openapi_str();
        let openapi_path = self.openapi_path.clone();
        let docs_path = self.docs_path.clone();
        let swagger_html = Self::swagger_html(&openapi_path);

        let host = self.host.clone().unwrap_or_else(|| "127.0.0.1".to_owned());
        let port = self.port.unwrap_or(6969);
        let addr: SocketAddr = format!("{}:{}", host, port).parse().expect("Invalid address");

        // --- 2. THE HANDLERS ---
        // We use `move` to pull `openapi_json` into the closure.
        // We add `axum::extract::State` as a parameter to help Rust's type inference
        // match the Handler<T, S> trait bound for your stateful router.
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

        // --- 3. REGISTER ROUTES ---
        let mut axum_router: Router<S> = Router::new();

        // Register user routers
        for api_router in self.routers {
            for route in api_router.routes {
                axum_router = axum_router.route(&route.path, route.handler);
                info!("Registering: {} {}", route.method, route.path);
            }
        }

        // Register the OpenAPI JSON route and Swagger UI
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

        // --- 4. FINALIZE AND SERVE ---
        let axum_router = axum_router
            .layer(middleware::from_fn(log_request))
            .layer(NormalizePathLayer::trim_trailing_slash())
            .with_state(self.state); // State S is consumed here

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        info!("Server listening on http://{}", addr);
        info!("Swagger UI available at http://{}{}", addr, docs_path);

        axum::serve(listener, axum_router).await.unwrap();
    }

}
