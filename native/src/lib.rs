extern crate reqwest;

#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

pub mod meetup;

fn hello(call: Call) -> JsResult<JsString> {
    let result = &format!("{:?}", meetup::get_events().unwrap());
    let scope = call.scope;
    Ok(JsString::new(scope, result).unwrap())
}

register_module!(m, {
    m.export("hello", hello)
});
