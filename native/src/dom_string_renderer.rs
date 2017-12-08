use std;
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

use util::dom_property::PROPERTIES;
use util::omitted_close_tags::OMITTED_CLOSE_TAGS;
use partial_renderer::{ReadSize, DomServerRenderer};


fn get_raw(scope: &mut RootScope, obj: Local, key: &str) -> Local {
    JsObject::from_raw(obj)
        .get(scope, key)
        .unwrap()
        .deref()
        .to_raw()
}

fn get_obj(scope: &mut RootScope, obj: Local, key: &str) -> JsObject {
    JsObject::from_raw(get_raw(scope, obj, key))
}

fn get_fn(scope: &mut RootScope, obj: Local, key: &str) -> JsFunction {
    JsFunction::<JsObject>::from_raw(get_raw(scope, obj, key))
}

fn to_string<T: Value>(scope: &mut RootScope, obj: T) -> String {
    obj.to_string(scope)
        .unwrap()
        .deref()
        .value()
}

fn get_children(scope: &mut RootScope, props: Local) -> Vec<JsValue> {
    let children_raw = get_raw(scope, props, "children");
    let val = JsValue::from_raw(children_raw).as_value(scope);
    let mut children = Vec::new();
    if val.is_a::<JsString>() {
        children.push(*val);
    } else if val.is_a::<JsArray>() {
        for v1 in JsArray::from_raw(val.to_raw()).to_vec(scope).unwrap() {
            if v1.is_a::<JsArray>() {
                children.extend(
                    JsArray::from_raw(v1.to_raw())
                        .to_vec(scope)
                        .unwrap()
                        .iter()
                        .map(|v| v.deref())
                );
            } else {
                children.push(*v1);
            }
        }
    }
    children
}

fn render_type(
    html: &mut String,
    scope: &mut RootScope,
    component: Local,
    level: usize
) {
    let prefix = std::iter::repeat("  ").take(level).collect::<String>();
    let type_raw = get_raw(scope, component, "type");
    let type_obj = JsObject::from_raw(type_raw);
    // println!(">>> type={}", to_string(scope, type_obj));
    let props = get_obj(scope, component, "props");
    // println!(">>> props={}", to_string(scope, props));
    let type_val = type_obj.as_value(scope);
    if type_val.is_a::<JsFunction>() {
        let props = props.as_value(scope);
        let instance: Handle<JsObject> = JsFunction::from_raw(type_raw)
            .construct(scope, vec![props])
            .unwrap();
        // println!(">>> instance={}", to_string(scope, *instance));
        let render_fn = get_fn(scope, instance.to_raw(), "render");
        let this = instance.as_value(scope);
        let obj = JsObject::from_raw(instance.to_raw()).as_value(scope);
        let rendered_component = render_fn
            .call(scope, this, vec![obj])
            .unwrap();
        // println!(">>> rendered_component={}", to_string(scope, *rendered_component));
        render_type(html, scope, rendered_component.deref().to_raw(), level);
    } else if type_val.is_a::<JsString>() {
        let tag = to_string(scope, type_obj);
        let mut header = format!("{}<{}", prefix, tag);
        let mut footer = String::new();
        if OMITTED_CLOSE_TAGS.contains(tag.as_str()) {
            header.push_str("/>");
            header.push_str("\n");
        } else {
            header.push_str(">");
            header.push_str("\n");
            footer = format!("{}</{}>", prefix, tag);
            footer.push_str("\n");
        }
        html.push_str(header.as_str());
        let children = get_children(scope, props.to_raw());
        for child in children {
            let child_val = child.as_value(scope);
            if child_val.is_a::<JsString>()
                || child_val.is_a::<JsNumber>()
                || child_val.is_a::<JsBoolean>()
            {
                let mut content = format!("{}  {}", prefix, to_string(scope, child));
                content.push_str("\n");
                html.push_str(content.as_str());
            } else if child_val.is_a::<JsObject>() {
                render_type(html, scope, child.to_raw(), level+1);
            } else {
                println!(">>> child={}", to_string(scope, child));
                panic!("Invalid child type");
            }
        }
        html.push_str(footer.as_str());
    } else {
        if JsValue::from_raw(component).as_value(scope).is_a::<JsArray>() {
            println!("component is a JsArray");
        }
        println!(">>> component={}", to_string(scope, JsObject::from_raw(component)));
        println!(">>> type={}", to_string(scope, type_obj));
        println!(">>> props={}", to_string(scope, props));
        let children = get_children(scope, props.to_raw());
        for child in children {
            println!(">>> child={}", to_string(scope, child));
        }
        panic!("Invalid component type");
    }
}

pub fn render_to_string(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let app = call
        .arguments
        .get(scope, 0)
        .unwrap();
    let mut html = String::new();
    let _ = render_type(&mut html, scope, app.to_raw(), 0);
    let element = ();
    let _bytes = DomServerRenderer::new(vec![element], false)
        .read(ReadSize::Infinity);
    Ok(JsString::new(scope, html.as_str()).unwrap())
}

pub fn render_to_static_markup(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let element = ();
    let bytes = DomServerRenderer::new(vec![element], true)
        .read(ReadSize::Infinity);
    let rv = String::from_utf8(bytes).unwrap();
    Ok(JsString::new(scope, rv.as_str()).unwrap())
}
