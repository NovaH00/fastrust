use axum::{routing::{get, post, put, patch, delete}, handler::Handler};
use crate::{
    route::{Method, Route},
    normalize_path
};
 

#[derive(Clone, Debug)]
pub struct APIRouter<S = ()> {
    pub prefix: String,
    pub routes: Vec<Route<S>>,
}

impl APIRouter<()> {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: normalize_path(prefix).to_owned(),
            routes: Vec::new(),
        }
    }
    
    // This is a debug function
    pub fn show_data(&self) {
        println!("Prefix: {}", self.prefix);
        for v in &self.routes {
            match v.method {
                Method::Get    => println!("GET {}", v.path),
                Method::Post   => println!("POST {}", v.path),
                Method::Put    => println!("PUT {}", v.path),
                Method::Patch  => println!("PATCH {}", v.path),
                Method::Delete => println!("DELETE {}", v.path)
            }
        } 
    }
    
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
    pub fn get<H, T>(&mut self, path: &str, handler: H) -> &Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let combined_path = self.prefix.clone() + path; 
        let route = Route { 
            method: Method::Get,
            path: normalize_path(&combined_path).to_owned(),
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
            path: normalize_path(&combined_path).to_owned(),
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
            path: normalize_path(&combined_path).to_owned(),
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
            path: normalize_path(&combined_path).to_owned(),
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
            path: normalize_path(&combined_path).to_owned(),
            handler: delete(handler) 
        };

        self.routes.push(route);
        self
    }
}

