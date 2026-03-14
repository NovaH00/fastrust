use axum::{
    routing::{get, post, put, patch, delete, head, connect, trace, options},
    handler::Handler
};
use crate::{
    route::{Method, Route},
    canonicalize_path
};

macro_rules! add_method {
    ($method:expr, $name:ident, $axum_fn:ident) => {
        pub fn $name<H, T>(&mut self, path: &str, handler: H) -> &mut Self
        where
            H: Handler<T, S>,
            T: 'static,
        {
            let combined_path = format!("{}{}", self.prefix, path);

            self.routes.push(Route {
                method: $method,
                path: canonicalize_path(&combined_path),
                handler: $axum_fn(handler),
            });

            self
        }
    };
}

#[derive(Clone, Debug)]
pub struct APIRouter<S = ()> {
    pub prefix: String,
    pub routes: Vec<Route<S>>,
}

impl<S> APIRouter<S>
where
    S: Clone + Send + Sync + 'static, // Axum requires these bounds for State
{
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: canonicalize_path(&prefix.into()),
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Route<S>) {
        self.routes.push(route);
    }

    pub fn include_router(&mut self, router: &APIRouter<S>) {
        for v in &router.routes {
            let combined_path = format!("{}{}", self.prefix, v.path);
            self.add_route(Route {
                method: v.method.clone(),
                path: combined_path,
                handler: v.handler.clone()
            });
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

