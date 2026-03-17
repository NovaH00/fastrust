use openapiv3 as oa;
use indexmap::IndexMap;
use crate::routing::Method;
use super::builder::APIApp;

impl<S> APIApp<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Generates the OpenAPI 3.0 specification as a JSON string.
    ///
    /// This method iterates through all registered routers and their routes,
    /// converting them into an OpenAPI 3.0 specification. The generated
    /// specification includes:
    ///
    /// - All registered paths and HTTP methods
    /// - Parameter definitions extracted from handler signatures
    /// - Request/response schemas when configured via `RouteConfig`
    /// - Server information if host/port are set
    ///
    /// # Returns
    ///
    /// A JSON-formatted string containing the OpenAPI 3.0 specification.
    /// Returns an empty string if serialization fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fastrust::{APIApp, APIRouter, RouteConfig};
    ///
    /// let mut api = APIRouter::new("/api");
    /// // Add routes...
    ///
    /// let app = APIApp::new().register_router(api);
    /// let openapi_json = app.generate_openapi_str();
    /// ```
    pub fn generate_openapi_str(&self) -> String {
        let mut paths: IndexMap<String, oa::ReferenceOr<oa::PathItem>> = IndexMap::new();

        for router in &self.routers {
            for route in &router.routes {
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

        let info = oa::Info {
            title: self.title.clone().unwrap_or_else(|| "Axum API".to_string()),
            description: self.description.clone(),
            version: self.version.clone(),
            contact: None,
            license: None,
            terms_of_service: None,
            extensions: IndexMap::new(),
        };

        let mut servers = Vec::new();
        if let (Some(host), Some(port)) = (&self.host, self.port) {
            servers.push(oa::Server {
                url: format!("http://{}:{}", host, port),
                description: Some("Application Server".to_string()),
                variables: None,
                extensions: IndexMap::new(),
            });
        }

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
}
