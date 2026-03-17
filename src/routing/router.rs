use axum::{
    routing::{get, post, put, patch, delete, head, connect, trace, options},
    handler::Handler
};
use crate::canonicalize_path;
use super::RouteConfig;
use super::route::{Route, Method};
use super::super::extractor::InspectSignature;

macro_rules! add_method {
    ($method:expr, $name:ident, $axum_fn:ident) => {
        pub fn $name<H, T>(&mut self, path: &str, handler: H, route_config: RouteConfig) -> &mut Self
        where
            H: Handler<T, S>,
            T: InspectSignature + 'static,
        {
            let args = T::extractors();

            let combined_path = format!("{}{}", self.prefix, path);

            let mut new_route = Route::new(
                $method,
                canonicalize_path(&combined_path),
                $axum_fn(handler)
            );

            new_route
                .set_openapi_operation(args)
                .set_route_config(route_config);

            self.routes.push(new_route);

            self
        }
    };
}

/// A router for defining and organizing API routes.
///
/// `APIRouter` provides a fluent interface for registering HTTP route handlers
/// with automatic OpenAPI documentation generation. Routes can be organized
/// using path prefixes and routers can be combined using `include_router()`.
///
/// # Type Parameters
///
/// * `S` - The application state type
///
/// # Examples
///
/// ```rust
/// use fastrust::{APIRouter, RouteConfig};
/// use axum::extract::Path;
///
/// async fn hello(Path(name): Path<String>) -> String {
///     format!("Hello {}", name)
/// }
///
/// let mut api = APIRouter::new("/api");
/// api.get("/hello/{name}", hello, RouteConfig::default().summary("Say hello"));
/// ```
///
/// # Path Prefixes
///
/// Routes registered with an `APIRouter` automatically have the router's
/// prefix prepended. For example, a router with prefix `/api` and a route
/// `/users` will result in the full path `/api/users`.
///
/// # Combining Routers
///
/// Routers can be combined using `include_router()`:
///
/// ```rust
/// use fastrust::APIRouter;
///
/// let mut api = APIRouter::new("/api");
/// let mut v1 = APIRouter::new("/v1");
///
/// v1.include_router(api); // Routes become /v1/api/...
/// ```
#[derive(Clone, Debug)]
pub struct APIRouter<S = ()> {
    /// The path prefix for all routes in this router.
    pub prefix: String,
    /// The list of routes registered in this router.
    pub routes: Vec<Route<S>>,
}

impl<S> APIRouter<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Creates a new `APIRouter` with the given path prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The path prefix for all routes (e.g., "/api", "/v1")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIRouter;
    ///
    /// let api = APIRouter::new("/api");
    /// let v1 = APIRouter::new("/v1");
    /// ```
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: canonicalize_path(&prefix.into()),
            routes: Vec::new(),
        }
    }

    /// Adds a route directly to the router.
    ///
    /// # Arguments
    ///
    /// * `route` - The route to add
    pub fn add_route(&mut self, route: Route<S>) {
        self.routes.push(route);
    }

    /// Includes all routes from another router into this one.
    ///
    /// This method consumes the provided router and adds all its routes
    /// to this router, prepending this router's prefix to each route's path.
    ///
    /// # Arguments
    ///
    /// * `router` - The router to include
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::APIRouter;
    ///
    /// let mut api = APIRouter::new("/api");
    /// let mut v1 = APIRouter::new("/v1");
    ///
    /// v1.include_router(api); // api routes now have /v1 prefix
    /// ```
    pub fn include_router(&mut self, router: APIRouter<S>) {
        for v in &router.routes {
            let combined_path = format!("{}{}", self.prefix, v.path);
            self.add_route(
                Route::new(
                    v.method.clone(),
                    combined_path,
                    v.handler.clone()
                )
            );
        }
    }

    add_method!(Method::Get, get, get);
    add_method!(Method::Post, post, post);
    add_method!(Method::Put, put, put);
    add_method!(Method::Patch, patch, patch);
    add_method!(Method::Delete, delete, delete);
    add_method!(Method::Head, head, head);
    add_method!(Method::Options, options, options);
    add_method!(Method::Trace, trace, trace);
    add_method!(Method::Connect, connect, connect);
}
