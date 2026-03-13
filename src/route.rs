use axum::routing::MethodRouter;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Method {
    Get, Post, Put, Patch, Delete
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
        };
        write!(f, "{s}")
    }
}


#[derive(Clone, Debug)]
pub struct Route<S = ()> {
    pub method: Method,
    pub path: String,
    pub handler: MethodRouter<S>
}


