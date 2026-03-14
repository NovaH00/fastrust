use axum::{routing::{get, post, put, patch, delete}, handler::Handler};
use crate::{
    route::{Method, Route},
    canonicalize_path
};
 

#[derive(Clone, Debug)]
pub struct APIRouter<S = ()> {
    pub prefix: String,
    pub routes: Vec<Route<S>>,
}

impl APIRouter<()> {
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn include_router(&mut self, router: &APIRouter) {
        for v in &router.routes {
            let combined_path = self.prefix.clone() + &v.path;
            self.add_route(Route {
                method: v.method.clone(),
                path: combined_path,
                handler: v.handler.clone()
            });
        } 
    }
}

impl<S> APIRouter<S>
where
    S: Clone + Send + Sync + 'static, // Axum requires these bounds for State
{
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: canonicalize_path(prefix),
            routes: Vec::new(),
        }
    }

    pub fn get<H, T>(&mut self, path: &str, handler: H) -> &Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let combined_path = self.prefix.clone() + path; 
        let route = Route { 
            method: Method::Get,
            path: canonicalize_path(&combined_path),
            handler: get(handler) 
        };
        self.routes.push(route);
        self
    }

    pub fn post<H, T>(&mut self, path: &str, handler: H) -> &Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let combined_path = self.prefix.clone() + path; 
        let route = Route { 
            method: Method::Post,
            path: canonicalize_path(&combined_path),
            handler: post(handler) 
        };

        self.routes.push(route);
        self
    }

    pub fn put<H, T>(&mut self, path: &str, handler: H) -> &Self
    where
        H: Handler<T, S>,
        T: 'static,
    {

        let combined_path = self.prefix.clone() + path; 
        let route = Route { 
            method: Method::Put,
            path: canonicalize_path(&combined_path),
            handler: put(handler) 
        };

        self.routes.push(route);
        self
    }

    pub fn patch<H, T>(&mut self, path: &str, handler: H) -> &Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let combined_path = self.prefix.clone() + path; 
        let route = Route { 
            method: Method::Patch,
            path: canonicalize_path(&combined_path),
            handler: patch(handler) 
        };

        self.routes.push(route);
        self
    }

    pub fn delete<H, T>(&mut self, path: &str, handler: H) -> &Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let combined_path = self.prefix.clone() + path; 
        let route = Route { 
            method: Method::Delete,
            path: canonicalize_path(&combined_path),
            handler: delete(handler) 
        };

        self.routes.push(route);
        self
    }
}

