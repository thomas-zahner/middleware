use extism_pdk::*;
use serde::{Deserialize, Serialize};

// ----------- BEGIN copy paste ----------------
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
// ----------- END copy paste ----------------

#[host_fn]
extern "ExtismHost" {
    fn next(request: Request) -> Response;
}

#[plugin_fn]
pub fn greet(request: Request) -> FnResult<Response> {
    let next_result = unsafe { next(request) };
    Ok(next_result?)
}
