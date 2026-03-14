use axum::{Router, middleware};
use tower_http::normalize_path::NormalizePathLayer;
use crate::router::APIRouter;
use crate::middleware::log_request;
use std::net::SocketAddr;
use tracing::{info, warn, error};


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

    pub async fn run(self) {
        tracing_subscriber::fmt().with_target(false).init();

        let host = match self.host {
            None => {
                info!("No host provided; defaulting to 127.0.0.1.");
                "127.0.0.1".to_owned()
            },
            Some(h) =>  h 
        }; 
        
        let port = match self.port {
            None => {
                info!("No port provided; defaulting to 6969.");
                6969
            },
            Some(p) => p 
        };

        // Parse socket address
        let addr: SocketAddr = format!("{}:{}", host, port)
            .parse()
            .unwrap_or_else(|err| {
                error!("Failed to parse socket address `{host}:{port}`; {err}");
                std::process::exit(1);
            });

        // Bind TCP listener
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .unwrap_or_else(|err| { 
                error!("Failed to bind TCP listener to `{addr}`; {err}");
                std::process::exit(1);
            });

        let mut axum_router: Router<S> = Router::new();

        if self.routers.is_empty() {
            warn!("No router registered; incoming requests will not be handled.")
        }
        else {
            info!("Registering paths from {} routers:", self.routers.len());
            for api_router in self.routers {
                for route in api_router.routes {
                    axum_router = axum_router.route(&route.path, route.handler);
                    info!("\t{} {}", route.method, route.path);
                }
            }
        }

        axum_router = axum_router
            .layer(middleware::from_fn(log_request))
            .layer(NormalizePathLayer::trim_trailing_slash());
 
        let axum_router = axum_router.with_state(self.state);

        info!("Server initialized.");
        info!("Server is listening on {}:{}.", host, port);
        axum::serve(listener, axum_router)
            .await
            .unwrap_or_else(|err| {
                error!("Axum server crashed; {err}");
                std::process::exit(1);
            });
    }
}
