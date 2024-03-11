pub(crate) trait Chainable<T> {
    fn handle(&mut self, input: T) -> T;
}

type Chain<T> = Vec<Box<dyn Chainable<T>>>;

fn traverse<T>(chain: Chain<T>, mut input: T) -> T {
    for mut e in chain {
        input = e.handle(input)
    }

    input
}

mod test {
    use super::{traverse, Chain, Chainable};

    struct Add(i64);

    struct Sub(i64);

    struct Request(i64);

    impl Chainable<Request> for Add {
        fn handle(&mut self, req: Request) -> Request {
            Request(req.0 + self.0)
        }
    }

    impl Chainable<Request> for Sub {
        fn handle(&mut self, req: Request) -> Request {
            Request(req.0 - self.0)
        }
    }

    #[test]
    fn example_chain() {
        let chain: Chain<Request> = vec![Box::new(Add(10)), Box::new(Sub(3))];
        let result = traverse(chain, Request(0));
        assert_eq!(result.0, 7);
    }
}
