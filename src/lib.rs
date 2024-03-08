use ::extism::{convert::Msgpack, FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use std::vec::IntoIter;

mod extism;
mod middlewares;

#[derive(ToBytes, Serialize, FromBytes, Deserialize, PartialEq, Eq, Hash, Clone)]
#[encoding(Msgpack)]
pub struct Request {
    url: String,
    // headers: Vec<String>,
}

#[derive(ToBytes, Serialize, FromBytes, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[encoding(Msgpack)]
pub struct Response {
    code: usize,
    // body: String,
}

pub type NextFunction = Box<dyn FnOnce(Request) -> Response>;

pub trait RequestMiddleware {
    fn handle(&mut self, req: Request, next: NextFunction) -> Response;
}

pub fn traverse(req: Request, mut iter: IntoIter<Box<dyn RequestMiddleware>>) -> Response {
    let middleware = iter.next();
    let get_next: NextFunction = Box::new(|req| traverse(req, iter));

    match middleware {
        Some(mut n) => n.handle(req, get_next),
        None => panic!("Called `next` function, but there was no next middleware"),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        middlewares::{Cache, Check, Rewrite},
        traverse, RequestMiddleware, Request, Response,
    };

    #[test]
    fn full_chain() {
        let rewrite = Rewrite {};
        let cache = Cache::default();
        let check = Check {};

        let chain: Vec<Box<dyn RequestMiddleware>> =
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
        impl RequestMiddleware for CheckMock {
            fn handle(&mut self, _: Request, _: crate::NextFunction) -> Response {
                Response { code: 200 }
            }
        }

        let cache = Cache::default();
        let check = CheckMock {};
        let chain: Vec<Box<dyn RequestMiddleware>> = vec![Box::new(cache), Box::new(check)];

        let req = Request {
            url: "example.com".into(),
        };

        let result = traverse(req, chain.into_iter());
        assert_eq!(result.code, 200);
    }
}
