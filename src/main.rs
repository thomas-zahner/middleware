use std::vec::IntoIter;

#[derive(Clone)]
struct Request {
    url: String,
    // headers: Vec<String>,
}

#[derive(Debug)]
struct Response {
    code: usize,
}

type NextFunction = Box<dyn FnOnce(Request) -> Response>;

trait Middleware {
    fn handle(&self, req: Request, next: NextFunction) -> Response;
}

struct Rewrite {}

impl Middleware for Rewrite {
    fn handle(&self, mut req: Request, next: NextFunction) -> Response {
        if req.url == "example.com" {
            req.url = "example.com/home".into();
        }
        next(req)
    }
}

struct Check {}

impl Middleware for Check {
    fn handle(&self, req: Request, _next: NextFunction) -> Response {
        println!("Checking {}", req.url);
        Response { code: 200 }
    }
}

fn main() {
    let rewrite = Rewrite {};
    let check = Check {};

    let chain: Vec<Box<dyn Middleware>> = vec![Box::new(rewrite), Box::new(check)];

    let req = Request {
        url: "example.com".into(),
    };

    let iter = chain.into_iter();
    let result = traverse(req, iter);

    println!("{:?}", result.code);
}

fn traverse(req: Request, mut iter: IntoIter<Box<dyn Middleware>>) -> Response {
    let next = iter.next();

    let f = Box::new(|req| traverse(req, iter));

    match next {
        Some(n) => n.handle(req, f),
        None => panic!("Called `next` function, but there was no next middleware"),
    }
}
