use std::{collections::HashMap, vec::IntoIter};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Request {
    url: String,
    // headers: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Response {
    code: usize,
    // body: String,
}

pub type NextFunction = Box<dyn FnOnce(Request) -> Response>;

pub trait Middleware {
    fn handle(&mut self, req: Request, next: NextFunction) -> Response;
}

struct Rewrite {}

impl Middleware for Rewrite {
    fn handle(&mut self, mut req: Request, next: NextFunction) -> Response {
        if req.url == "example.com" {
            req.url = "example.com/home".into();
        }
        next(req)
    }
}

struct Cache {
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

struct Check {}

impl Middleware for Check {
    fn handle(&mut self, req: Request, _: NextFunction) -> Response {
        println!("Checking {}", req.url);
        Response { code: 200 }
    }
}

pub fn traverse(req: Request, mut iter: IntoIter<Box<dyn Middleware>>) -> Response {
    let middleware = iter.next();
    let get_next: NextFunction = Box::new(|req| traverse(req, iter));

    match middleware {
        Some(mut n) => n.handle(req, get_next),
        None => panic!("Called `next` function, but there was no next middleware"),
    }
}

#[cfg(test)]
mod test {
    use crate::{traverse, Cache, Check, Middleware, Request, Response, Rewrite};
    use std::collections::HashMap;

    #[test]
    fn full_chain() {
        let rewrite = Rewrite {};
        let cache = Cache {
            inner: HashMap::new(),
        };
        let check = Check {};

        let chain: Vec<Box<dyn Middleware>> =
            vec![Box::new(rewrite), Box::new(cache), Box::new(check)];

        let req = Request {
            url: "example.com".into(),
        };

        let result = traverse(req, chain.into_iter());
        assert_eq!(result.code, 200);
    }

    #[test]
    fn caching() {
        struct CheckMock {}
        impl Middleware for CheckMock {
            fn handle(&mut self, _: Request, _: crate::NextFunction) -> Response {
                Response { code: 200 }
            }
        }

        let cache = Cache {
            inner: HashMap::new(),
        };
        let check = CheckMock {};
        let chain: Vec<Box<dyn Middleware>> = vec![Box::new(cache), Box::new(check)];

        let req = Request {
            url: "example.com".into(),
        };

        let result = traverse(req, chain.into_iter());
        assert_eq!(result.code, 200);
    }
}
