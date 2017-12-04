#[macro_use]
extern crate neon;
extern crate neon_runtime;

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

mod dom_string_renderer;
mod partial_renderer;

fn get_obj(scope: &mut RootScope, obj: Local, key: &str) -> JsObject {
    JsObject::from_raw(
        JsObject::from_raw(obj)
            .get(scope, key)
            .unwrap()
            .deref()
            .to_raw()
    )
}

fn get_fn(scope: &mut RootScope, obj: Local, key: &str) -> JsFunction {
    JsFunction::<JsObject>::from_raw(
        JsObject::from_raw(obj)
            .get(scope, key)
            .unwrap()
            .deref()
            .to_raw()
    )
}

fn to_string<T: Value>(scope: &mut RootScope, obj: T) -> String {
    obj.to_string(scope)
        .unwrap()
        .deref()
        .value()
}

fn render_to_string(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let app = call
        .arguments
        .get(scope, 0)
        .unwrap();
    let app_class = get_obj(scope, app.to_raw(), "type");
    println!(">>> app_class={}", to_string(scope, app_class));
    let app_call_fn = get_fn(scope, app_class.to_raw(), "call");
    let prototype = get_obj(scope, app_class.to_raw(), "prototype");
    println!(">>> prototype={}", to_string(scope, prototype));
    let this = app_class.as_value(scope);
    let obj = JsObject::new(scope);
    obj.set("__proto__", prototype.as_value(scope)).unwrap();
    let instance = app_call_fn
        .call(scope, this, vec![obj])
        .map_err(|e| {
            println!("[Error]: {:?}", e);
            ()
        })
        .unwrap();
    println!(">>> instance={}", to_string(scope, *instance));
    let render_fn = get_fn(scope, instance.to_raw(), "render");
    println!(">>> render_fn={}", to_string(scope, render_fn));
    let this = instance.as_value(scope);
    let obj = JsObject::from_raw(instance.to_raw()).as_value(scope);
    let rendered_app = render_fn
        .call(scope, this, vec![obj])
        .map_err(|e| {
            println!("[Error]: description={:?}, cause={:?}, e={}",
                     e.description(), e.cause(), e);
            ()
        })
        .unwrap();
    println!(">>> rendered_app={}", to_string(scope, *rendered_app));
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
