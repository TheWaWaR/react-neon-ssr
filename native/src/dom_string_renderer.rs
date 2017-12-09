use neon::vm::{Call, JsResult};
use neon::js::{
    JsString,
};

use partial_renderer::{ReadSize, DomServerRenderer};

pub fn render_to_string(call: Call) -> JsResult<JsString> {
    let mut renderer = DomServerRenderer::new(call, false);
    let html = renderer
        .read(ReadSize::Infinity)
        .unwrap_or_else(|| "".to_string());
    Ok(JsString::new(renderer.call.scope, html.as_str()).unwrap())
}

pub fn render_to_static_markup(call: Call) -> JsResult<JsString> {
    let mut renderer = DomServerRenderer::new(call, true);
    let html = renderer
        .read(ReadSize::Infinity)
        .unwrap_or_else(|| "".to_string());
    Ok(JsString::new(renderer.call.scope, html.as_str()).unwrap())
}
