use crate::APIRouter;

/// The main application struct for fastrust.
///
/// `APIApp` is the entry point for building a fastrust web application.
/// It provides a builder pattern for configuring the application and
/// automatically generates OpenAPI documentation.
///
/// # Type Parameters
///
/// * `S` - The application state type. Use `()` for no state.
///
/// # Examples
///
/// ## Basic usage without state
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
/// ## With application state
///
/// ```rust,no_run
/// use fastrust::{APIApp, APIRouter, RouteConfig};
///
/// #[derive(Clone)]
/// struct AppState {
///     counter: std::sync::Arc<std::sync::Mutex<i32>>,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let state = AppState {
///         counter: std::sync::Arc::new(std::sync::Mutex::new(0)),
///     };
///
///     let mut api = APIRouter::new("/api");
///     // Add routes...
///
///     APIApp::new_with_state(state)
///         .set_title("My API")
///         .register_router(api)
///         .run().await;
/// }
/// ```
///
/// # OpenAPI Documentation
///
/// By default, the OpenAPI specification is available at `/openapi.json`
/// and the Swagger UI is available at `/docs`. These paths can be
/// customized using [`set_openapi_path`](Self::set_openapi_path) and
/// [`set_docs_path`](Self::set_docs_path).
#[derive(Debug)]
pub struct APIApp<S = ()>
where
    S: Clone + Send + Sync + 'static,
{
    pub(crate) title:        Option<String>,
    pub(crate) summary:      Option<String>,
    pub(crate) description:  Option<String>,
    pub(crate) version:      String,
    pub(crate) openapi_path: String,
    pub(crate) docs_path:    String,
    pub(crate) state:        S,

    pub(crate) host:         Option<String>,
    pub(crate) port:         Option<i32>,

    pub(crate) routers:      Vec<APIRouter<S>>,
}

impl Default for APIApp<()> {
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
    /// Creates a new `APIApp` with no state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// let app = APIApp::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S> APIApp<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a new `APIApp` with the provided state.
    ///
    /// # Arguments
    ///
    /// * `state` - The application state to be shared across handlers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// #[derive(Clone)]
    /// struct AppState {
    ///     value: i32,
    /// }
    ///
    /// let state = AppState { value: 42 };
    /// let app = APIApp::new_with_state(state);
    /// ```
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

    /// Sets the API title.
    ///
    /// The title is displayed in the Swagger UI and used in the OpenAPI
    /// specification.
    ///
    /// # Arguments
    ///
    /// * `title` - The API title
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// let app = APIApp::new().set_title("My Awesome API");
    /// ```
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    /// Sets the API summary.
    ///
    /// A short description of the API, displayed in the Swagger UI.
    ///
    /// # Arguments
    ///
    /// * `summary` - The API summary
    pub fn set_summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_owned());
        self
    }

    /// Sets the API description.
    ///
    /// A longer description of the API. Supports Markdown formatting.
    ///
    /// # Arguments
    ///
    /// * `description` - The API description
    pub fn set_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }

    /// Sets the API version.
    ///
    /// # Arguments
    ///
    /// * `version` - The API version string (e.g., "1.0.0")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// let app = APIApp::new().set_version("1.0.0");
    /// ```
    pub fn set_version(mut self, version: &str) -> Self {
        self.version = version.to_owned();
        self
    }

    /// Sets the OpenAPI specification endpoint path.
    ///
    /// # Arguments
    ///
    /// * `openapi_path` - The path for the OpenAPI JSON (default: "/openapi.json")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// let app = APIApp::new().set_openapi_path("/api-spec.json");
    /// ```
    pub fn set_openapi_path(mut self, openapi_path: &str) -> Self {
        self.openapi_path = openapi_path.to_owned();
        self
    }

    /// Sets the Swagger UI documentation path.
    ///
    /// # Arguments
    ///
    /// * `docs_path` - The path for the Swagger UI (default: "/docs")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIApp;
    ///
    /// let app = APIApp::new().set_docs_path("/swagger");
    /// ```
    pub fn set_docs_path(mut self, docs_path: &str) -> Self {
        self.docs_path = docs_path.to_owned();
        self
    }

    /// Sets the server host.
    ///
    /// The host is used in the OpenAPI specification to generate the
    /// server URL.
    ///
    /// # Arguments
    ///
    /// * `host` - The server host (e.g., "localhost", "api.example.com")
    pub fn set_host(mut self, host: &str) -> Self {
        self.host = Some(host.to_owned());
        self
    }

    /// Sets the server port.
    ///
    /// # Arguments
    ///
    /// * `port` - The server port number (default: 6969)
    pub fn set_port(mut self, port: i32) -> Self {
        self.port = Some(port);
        self
    }

    /// Registers a router with the application.
    ///
    /// Multiple routers can be registered. They are processed in the
    /// order they are registered.
    ///
    /// # Arguments
    ///
    /// * `router` - An [`APIRouter`] containing route definitions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::{APIApp, APIRouter, RouteConfig};
    ///
    /// let mut api = APIRouter::new("/api");
    /// // Add routes to api...
    ///
    /// let app = APIApp::new().register_router(api);
    /// ```
    pub fn register_router(mut self, router: APIRouter<S>) -> Self {
        self.routers.push(router);
        self
    }
}
