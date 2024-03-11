use std::collections::HashMap;

use crate::middleware::{Middleware, NextFunction, Request, Response};

pub struct Rewrite {}

impl Middleware for Rewrite {
    fn handle(&mut self, mut req: Request, next: NextFunction) -> Response {
        if req.url == "example.com" {
            req.url = "example.com/home".into();
        }
        next(req)
    }
}

pub struct Check {}

impl Middleware for Check {
    fn handle(&mut self, req: Request, _: NextFunction) -> Response {
        println!("Checking {}", req.url);
        Response { code: 200 }
    }
}

#[derive(Default)]
pub struct Cache {
    inner: HashMap<Request, Response>,
}

impl Middleware for Cache {
    fn handle(&mut self, req: Request, next: NextFunction) -> Response {
        match self.inner.get(&req) {
            Some(res) => res.clone(),
            None => {
                let res = next(req.clone());
                self.inner.insert(req, res.clone());
                res
            }
        }
    }
}
