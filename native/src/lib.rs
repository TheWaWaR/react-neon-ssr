#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

mod dom_string_renderer;
mod partial_renderer;

use partial_renderer::{
    ReadSize,
    DomServerRenderer,
};

fn render_to_string(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let mut rv = String::from_utf8(
        dom_string_renderer::render_to_string(())
    ).unwrap();
    rv.push_str("~render_to_string");
    Ok(JsString::new(scope, rv.as_str()).unwrap())
}

fn render_to_static_markup(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let mut rv = String::from_utf8(
        dom_string_renderer::render_to_static_markup(())
    ).unwrap();
    rv.push_str("~render_to_static_markup");
    Ok(JsString::new(scope, rv.as_str()).unwrap())
}

register_module!(m, {
    m.export("renderToString", render_to_string)?;
    m.export("renderToStaticMarkup", render_to_static_markup)?;
    Ok(())
});
