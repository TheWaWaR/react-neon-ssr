use std;
use std::ops::Deref;
use std::collections::HashSet;

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

use util::{
    get_raw,
    get_obj,
    get_fn,
    to_string,
    is_custom_component,
    escape_text_content_for_browser,
};
use util::dom_property::PROPERTIES;
use util::omitted_close_tags::OMITTED_CLOSE_TAGS;
use dom_markup_operations::{
    create_markup_for_root,
    create_markup_for_property,
    create_markup_for_custom_attribute,
};

const STYLE: &str = "style";

lazy_static! {
    static ref RESERVED_PROPS: HashSet<&'static str> = hashset! {
        "children",
        "dangerouslySetInnerHTML",
        "suppressContentEditableWarning",
        "suppressHydrationWarning",
    };
}


fn get_children(scope: &mut RootScope, props: Local) -> Vec<JsValue> {
    let children_raw = get_raw(scope, props, "children");
    let val = JsValue::from_raw(children_raw).as_value(scope);
    let mut children = Vec::new();
    if val.is_a::<JsArray>() {
        for inner_val in JsArray::from_raw(val.to_raw()).to_vec(scope).unwrap() {
            if inner_val.is_a::<JsArray>() {
                children.extend(
                    JsArray::from_raw(inner_val.to_raw())
                        .to_vec(scope)
                        .unwrap()
                        .iter()
                        .map(|v| v.deref())
                );
            } else if inner_val.is_a::<JsObject>()
                || inner_val.is_a::<JsString>()
                || inner_val.is_a::<JsNumber>()
            {
                children.push(*inner_val);
            } else if inner_val.is_a::<JsUndefined>()
                || inner_val.is_a::<JsNull>()
                || inner_val.is_a::<JsBoolean>()
            {
            } else {
                println!(
                    "[WARN]: Inner Children={}",
                    to_string(scope, JsValue::from_raw(inner_val.to_raw()))
                );
                // panic!("Unexpected children type");
            }
        }
    } else if val.is_a::<JsObject>()
        || val.is_a::<JsString>()
        || val.is_a::<JsNumber>()
    {
        children.push(*val);
    } else if val.is_a::<JsUndefined>()
        || val.is_a::<JsNull>()
        || val.is_a::<JsBoolean>()
    {
    } else {
        println!(
            "[WARN]: Children={}",
            to_string(scope, JsValue::from_raw(children_raw))
        );
        // panic!("Unexpected children type");
    }
    children
}

fn create_markup_for_styles(
    scope: &mut RootScope,
    styles: Handle<JsValue>,
) -> Option<String> {
    None
}

fn get_non_children_inner_markup(
    scope: &mut RootScope,
    props: Local,
    children: &Vec<JsValue>,
) -> Option<String> {
    let inner_html = get_raw(scope, props, "dangerouslySetInnerHTML");
    if JsValue::from_raw(inner_html).as_value(scope).is_a::<JsObject>() {
        let the_html = get_raw(scope, inner_html, "__html");
        if !JsValue::from_raw(the_html).as_value(scope).is_a::<JsNull>() {
            return Some(to_string(scope, JsValue::from_raw(the_html)));
        }
    } else {
        if children.len() == 1 {
            let content: Handle<JsValue> = children[0].as_value(scope);
            if content.is_a::<JsString>() || content.is_a::<JsNumber>() {
                return Some(escape_text_content_for_browser(scope, content));
            }
        }
    }
    None
}

fn create_open_tag_markup(
    scope: &mut RootScope,
    tag_verbatim: &str,
    tag_lowercase: &str,
    props: JsObject,
    namespace: &str,
    make_static_markup: bool,
    is_root_element: bool,
) -> String {
    let mut ret = format!("<{}", tag_verbatim);
    let own_property_names = props
        .get_own_property_names(scope)
        .unwrap()
        .to_vec(scope)
        .unwrap();
    for prop_key in own_property_names {
        let mut prop_key: String = prop_key
            .downcast::<JsString>()
            .unwrap()
            .value();
        let mut prop_value: Handle<JsValue> = props
            .get(scope, prop_key.as_str())
            .unwrap();
        if prop_value.is_a::<JsNull>() {
            continue;
        }
        if prop_key == STYLE {
            prop_value = create_markup_for_styles(scope, prop_value)
                .map(|value| {
                    JsString::new(scope, value.as_str())
                        .unwrap()
                        .as_value(scope)
                })
                .unwrap_or_else(|| {
                    JsNull::new().as_value(scope)
                });
        }
        let mut markup: Option<String> = None;
        if is_custom_component(tag_lowercase, props.as_value(scope)) {
            if !RESERVED_PROPS.contains(prop_key.as_str()) {
                markup = Some(create_markup_for_custom_attribute(
                    scope, prop_key.as_str(), prop_value
                ));
            }
        } else {
            markup = create_markup_for_property(
                scope, prop_key.as_str(), prop_value
            )
        }
        if let Some(markup) = markup {
            ret.push_str(format!(" {}", markup).as_str());
        }
    }

    // For static pages, no need to put React ID and checksum. Saves lots of
    // bytes.
    if make_static_markup {
        return ret;
    }
    if is_root_element {
        ret.push_str(" ");
        ret.push_str(create_markup_for_root().as_str());
    }
    ret
}

fn render_type(
    html: &mut String,
    scope: &mut RootScope,
    component: Local,
    static_markup: bool,
    level: u32,
) {
    let type_raw = get_raw(scope, component, "type");
    let type_obj = JsObject::from_raw(type_raw);
    let props = get_obj(scope, component, "props");
    let type_val = type_obj.as_value(scope);
    if type_val.is_a::<JsFunction>() {
        let props = props.as_value(scope);
        let instance: Handle<JsObject> = JsFunction::from_raw(type_raw)
            .construct(scope, vec![props])
            .unwrap();
        let render_fn = get_fn(scope, instance.to_raw(), "render");
        let this = instance.as_value(scope);
        let obj = JsObject::from_raw(instance.to_raw()).as_value(scope);
        let rendered_component = render_fn
            .call(scope, this, vec![obj])
            .unwrap();
        render_type(html, scope, rendered_component.deref().to_raw(), static_markup, level+1);
    } else if type_val.is_a::<JsString>() {
        let type_str = to_string(scope, type_obj);
        let tag = type_str.to_lowercase();

        let mut header = create_open_tag_markup(
            scope,
            type_str.as_str(),
            tag.as_str(),
            props,
            "",
            static_markup,
            level == 0,
        );
        let mut footer = String::new();
        if OMITTED_CLOSE_TAGS.contains(tag.as_str()) {
            header.push_str("/>");
        } else {
            header.push_str(">");
            footer = format!("</{}>", type_str);
        }
        html.push_str(header.as_str());
        let children = get_children(scope, props.to_raw());
        if let Some(content) = get_non_children_inner_markup(
            scope, props.to_raw(), &children
        ) {
            html.push_str(content.as_str());
        } else {
            for child in children {
                let child_val = child.as_value(scope);
                if child_val.is_a::<JsString>()
                    || child_val.is_a::<JsNumber>()
                    || child_val.is_a::<JsBoolean>()
                {
                    let mut content = to_string(scope, child);
                    html.push_str(content.as_str());
                } else if child_val.is_a::<JsObject>() {
                    render_type(html, scope, child.to_raw(), static_markup, level+1);
                } else {
                    println!(">>> child={}", to_string(scope, child));
                    panic!("Invalid child type");
                }
            }
        };
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

pub enum ReadSize {
    Infinity,
    // How many bytes to read
    Size(usize),
}

pub struct DomServerRenderer<'a> {
    pub call: Call<'a>,
    static_markup: bool,
}

impl<'a> DomServerRenderer<'a> {
    pub fn new(call: Call<'a>, static_markup: bool) -> Self {
        DomServerRenderer{call, static_markup}
    }

    pub fn read(&mut self, size: ReadSize) -> Option<String> {
        let mut html = String::new();
        let component = self.call
            .arguments
            .get(self.call.scope, 0)
            .unwrap()
            .to_raw();
        render_type(&mut html, self.call.scope, component, self.static_markup, 0);
        Some(html)
    }
}
