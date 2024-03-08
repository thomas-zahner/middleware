use std::collections::HashMap;

use crate::{extism::call, RequestMiddleware, NextFunction, Request, Response};

pub(crate) struct Rewrite {}

impl RequestMiddleware for Rewrite {
    fn handle(&mut self, mut req: Request, next: NextFunction) -> Response {
        if req.url == "example.com" {
            req.url = "example.com/home".into();
        }
        next(req)
    }
}

pub(crate) struct Check {}

impl RequestMiddleware for Check {
    fn handle(&mut self, req: Request, _: NextFunction) -> Response {
        println!("Checking {}", req.url);
        Response { code: 200 }
    }
}

#[derive(Default)]
pub(crate) struct Cache {
    inner: HashMap<Request, Response>,
}

impl RequestMiddleware for Cache {
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

pub(crate) struct ExtismMiddleware {}

impl RequestMiddleware for ExtismMiddleware  {
    fn handle(&mut self, mut req: Request, next: NextFunction) -> Response {
        call(req, next)
    }
}
