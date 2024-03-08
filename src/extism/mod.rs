use extism::*;

use crate::{NextFunction, Request, Response};

pub(crate) fn call(req: Request, next_function: NextFunction) -> Response {
    host_fn!(next(req: Request) -> Response {
        todo!()
    });
    // host_fn!(next()->Response {
    //     next_function
    // });

    let url = Wasm::file("my-plugin/target/wasm32-unknown-unknown/debug/my_plugin.wasm");
    let manifest = Manifest::new([url]);
    let mut plugin = PluginBuilder::new(manifest)
        .with_wasi(true)
        .with_function("next", [PTR], [PTR], UserData::new(()), next)
        .build()
        .unwrap();
    plugin.call::<Request, Response>("greet", req).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{Request, Response};

    use super::call;

    #[test]
    fn basic_usage() {
        let req = Request {
            url: "example.com".into(),
        };

        let next = Box::new(|req: Request| Response { code: 200 });
        let result = call(req, next);
        assert_eq!(result, Response { code: 200 });
    }
}
