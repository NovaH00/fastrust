use axum::{
    routing::{get, post, put, patch, delete, head, connect, trace, options},
    handler::Handler
};
use crate::{
    canonicalize_path
};
use openapiv3::PathItem;
use openapiv3 as oa;
use indexmap::IndexMap;
use std::collections::BTreeMap;
use schemars::JsonSchema;
use schemars::Schema as JsonSchemaObject;
use super::RouteConfig;
use super::route::{Route, Method};
use super::super::inspector::InspectSignature;

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


/// Represents a router that can bind methods (GET, POST,...)
/// NOTICED: The self.routes contains routes that HAVE BEEN added the prefix,
/// no no manually adding required. The the self.prefix is for tooling/data purpose
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

    /// Consumes the provided router, adds the routes in that router to self. 
    /// Also adds the self.prefix to the routes in the consumed router. 
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

    pub fn get_dbg<T, H>(&mut self, path: &str, handler: H, route_config: RouteConfig) -> &mut Self
    where
        H: Handler<T, S>,
        T: InspectSignature + 'static,
    {
        let args = T::extractors();
       
        let combined_path = format!("{}{}", self.prefix, path);

        let mut new_route = Route::new(
            Method::Get,
            canonicalize_path(&combined_path),
            get(handler)
        );

        new_route
            .set_openapi_operation(args)
            .set_route_config(route_config);

        self.routes.push(new_route);

        self
    }

    pub fn post_dbg<T, H>(&mut self, path: &str, handler: H, route_config: RouteConfig) -> &mut Self
    where
        H: Handler<T, S>,
        T: InspectSignature + 'static,
    {
        let args = T::extractors();
       
        let combined_path = format!("{}{}", self.prefix, path);

        let mut new_route = Route::new(
            Method::Post,
            canonicalize_path(&combined_path),
            post(handler)
        );

        new_route
            .set_openapi_operation(args)
            .set_route_config(route_config);

        self.routes.push(new_route);

        self
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

