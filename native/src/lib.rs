#[macro_use]
extern crate neon;
extern crate neon_runtime;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

mod dom_string_renderer;
mod partial_renderer;
mod util;

use std::ops::Deref;
use std::error::Error;

use neon_runtime::raw::Local;
use neon::scope::{RootScope, Scope};
use neon::vm::{This, Call, JsResult};
use neon::mem::{
    Managed, Handle
};
use neon::js::class::{Class, JsClass};
use neon::js::{Value, Object};
use neon::js::{
    JsArray,
    JsBoolean,
    JsFunction,
    JsNull,
    JsNumber,
    JsObject,
    JsString,
    JsUndefined,
    JsValue,
};

use dom_string_renderer::{
    render_to_string,
    render_to_static_markup,
};


register_module!(m, {
    m.export("renderToString", render_to_string)?;
    m.export("renderToStaticMarkup", render_to_static_markup)?;
    Ok(())
});
