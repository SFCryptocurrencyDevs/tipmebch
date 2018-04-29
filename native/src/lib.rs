extern crate reqwest;
extern crate serde_json;
extern crate ws;

#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

pub mod meetup;

fn stream(call: Call) -> JsResult<(JsString)> {
    meetup::connect_to_stream();
    let scope = call.scope;
    Ok(JsString::new(scope, "hello").unwrap())
}

register_module!(m, {
    try!(m.export("stream", stream));
    Ok(())
});
