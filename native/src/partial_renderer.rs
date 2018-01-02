use std::cmp::{PartialOrd, Ordering};
use std::time::{Instant, Duration};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::collections::{HashSet, HashMap};

use regex::{Regex, Captures};
use neon_runtime::raw::Local;
use neon::scope::{RootScope, Scope};
use neon::vm::{This, Call, JsResult};
use neon::mem::{
    Managed, Handle
};
use neon::js::class::{Class, JsClass};
use neon::js::{Value, Variant, Object};
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
    duration_str,
    not,
    get_raw,
    get_obj,
    get_fn,
    to_string,

    hyphenate_style_name,

    get_intrinsic_namespace,
    get_child_namespace,
    is_custom_component,
    escape_text_content_for_browser,
    dangerous_style_value,
};
use util::dom_namespaces::{
    HTML_NAMESPACE,
    NAMESPACES
};
use util::dom_property::PROPERTIES;
use util::omitted_close_tags::OMITTED_CLOSE_TAGS;
use dom_markup_operations::{
    create_markup_for_root,
    create_markup_for_property,
    create_markup_for_custom_attribute,
};

const STYLE: &str = "style";
const VALUE_PROP_NAMES: [&'static str; 2] = ["value", "defaultValue"];

lazy_static! {
    static ref RESERVED_PROPS: HashSet<&'static str> = hashset! {
        "children",
        "dangerouslySetInnerHTML",
        "suppressContentEditableWarning",
        "suppressHydrationWarning",
    };
    static ref NEWLINE_EATING_TAGS: HashSet<&'static str> = hashset! {
        "listing", "pre", "textarea",
    };
    static ref STYLE_NAME_CACHE: Arc<Mutex<HashMap<String, String>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
    // We accept any tag to be rendered but since this gets injected into arbitrary
    // HTML, we want to make sure that it's a safe tag.
    // http://www.w3.org/TR/REC-xml/#NT-Name
    static ref VALID_TAG_REGEX: Regex = {
        Regex::new(r"^[a-zA-Z][a-zA-Z:_\.\-\d]*$").unwrap() // Simplified subset
    };
    static ref VALIDATED_TAG_CACHE: Arc<Mutex<HashSet<String>>> = {
        Arc::new(Mutex::new(HashSet::new()))
    };
}


fn get_children(scope: &mut RootScope, props: Local) -> Vec<JsValue> {
    let children_raw = get_raw(scope, props, "children");
    let val = JsValue::from_raw(children_raw).as_value(scope);
    let mut children = Vec::new();
    // NOTE: Can not use variant() here, beacuse a JsArray is also a JsObject.
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
                    to_string(scope, &JsValue::from_raw(inner_val.to_raw()))
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
            to_string(scope, &JsValue::from_raw(children_raw))
        );
        // panic!("Unexpected children type");
    }
    children
}

fn validate_dangerous_tag(tag: &str) {
    let mut cache = VALIDATED_TAG_CACHE.lock().unwrap();
    if !cache.contains(tag) {
        debug_assert!(
            VALID_TAG_REGEX.is_match(tag),
            format!("Invalid tag: {}", tag)
        );
        cache.insert(tag.to_string());
    }
}

fn should_construct(
    scope: &mut RootScope,
    component: &Handle<JsValue>
) -> bool {
    let prototype = JsObject
        ::from_raw(component.to_raw())
        .get(scope, "prototype")
        .unwrap();
    if prototype.is_a::<JsObject>() {
        let is_react_component = JsObject
            ::from_raw(prototype.to_raw())
            .get(scope, "isReactComponent")
            .unwrap();
        !not(is_react_component)
    } else {
        false
    }
}

fn get_component_name(
    scope: &mut RootScope,
    type_val: Handle<JsValue>
) -> Option<String> {
    match type_val.variant() {
        Variant::String(s) => Some(s.value()),
        Variant::Function(f) => {
            let f_obj = JsObject::from_raw(f.to_raw());
            let display_name = f_obj
                .get(scope, "displayName")
                .unwrap();
            if display_name.is_a::<JsString>() {
                Some(to_string(scope, display_name.deref()))
            } else {
                let name = f_obj
                    .get(scope, "name")
                    .unwrap();
                if name.is_a::<JsString>() {
                    Some(to_string(scope, name.deref()))
                } else {
                    None
                }
            }
        },
        _ => None
    }
}

fn process_style_name(name: &str) -> String {
    let mut cache = STYLE_NAME_CACHE.lock().unwrap();
    if !cache.contains_key(name) {
        cache.insert(name.to_string(), hyphenate_style_name(name));
    }
    cache.get(name).map(|s| s.clone()).unwrap()
}

fn create_markup_for_styles(
    scope: &mut RootScope,
    styles: Handle<JsValue>,
) -> Option<String> {
    let mut serialized = String::new();
    let mut is_first = true;
    let own_property_names = styles.downcast::<JsObject>()
        .unwrap()
        .get_own_property_names(scope)
        .unwrap()
        .to_vec(scope)
        .unwrap();
    for style_name in own_property_names {
        let style_name = style_name
            .downcast::<JsString>()
            .unwrap()
            .value();
        let is_custom_property = style_name.starts_with("--");
        let style_value = get_raw(
            scope, styles.to_raw(), style_name.as_str()
        );
        let style_value_handle = JsValue::from_raw(style_value).as_value(scope);
        if !style_value_handle.is_a::<JsNull>() {
            let delimiter = if is_first {""} else {";"};
            let name = process_style_name(style_name.as_str());
            let value = dangerous_style_value(
                style_name.as_str(),
                style_value_handle,
                is_custom_property,
            );
            serialized.push_str(format!("{}{}:{}", delimiter, name, value).as_str());
            is_first = false;
        }
    }
    if serialized.len() > 0 { Some(serialized) } else { None }
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
            return Some(to_string(scope, &JsValue::from_raw(the_html)));
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
            );
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

fn resolve(
    scope: &mut RootScope,
    child: Local,
    context: JsObject
) -> (Local, JsObject) {
    let type_raw = get_raw(scope, child, "type");
    let type_obj = JsObject::from_raw(type_raw);
    let props = get_obj(scope, child, "props");
    let type_val = type_obj.as_value(scope);
    let mut rendered_component: Option<Local> = None;
    if type_val.is_a::<JsFunction>() {
        let props = props.as_value(scope);
        let instance: Handle<JsObject> = JsFunction::from_raw(type_raw)
            .construct(scope, vec![props])
            .unwrap();
        let render_fn = get_fn(scope, instance.to_raw(), "render");
        let this = instance.as_value(scope);
        let obj = JsObject::from_raw(instance.to_raw()).as_value(scope);
        rendered_component = Some(
            render_fn
                .call(scope, this, vec![obj])
                .unwrap()
                .to_raw()
        );
    }
    (rendered_component.unwrap(), context)
}

fn render_type(
    html: &mut String,
    scope: &mut RootScope,
    component: Local,
    static_markup: bool,
    previous_was_text_node: bool,
    level: u32,
) -> Duration {
    let mut render_cost: Duration = Duration::from_secs(0);
    let mut current_is_text_node = false;
    let type_raw = get_raw(scope, component, "type");
    let type_obj = JsObject::from_raw(type_raw);
    let props = get_obj(scope, component, "props");
    let type_val = type_obj.as_value(scope);
    if type_val.is_a::<JsFunction>() {
        let props = props.as_value(scope);
        let now = Instant::now();
        let instance: Handle<JsObject> = JsFunction::from_raw(type_raw)
            .construct(scope, vec![props])
            .unwrap();
        let render_fn = get_fn(scope, instance.to_raw(), "render");
        let this = instance.as_value(scope);
        let obj = JsObject::from_raw(instance.to_raw()).as_value(scope);
        let rendered_component = render_fn
            .call(scope, this, vec![obj])
            .unwrap();
        render_cost += now.elapsed();
        render_cost += render_type(
            html,
            scope,
            rendered_component.deref().to_raw(),
            static_markup,
            current_is_text_node,
            level+1
        );
    } else if type_val.is_a::<JsString>() {
        let type_str = to_string(scope, &type_obj);
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
        // FIXME:
        if let Some(content) = get_non_children_inner_markup(
            scope, props.to_raw(), &children
        ) {
            if NEWLINE_EATING_TAGS.contains(tag.as_str())
                && content.chars().nth(0) == Some('\n')
            {
                html.push_str("\n");
            }
            html.push_str(content.as_str());
        } else {
            for child in children {
                let child_val = child.as_value(scope);
                if child_val.is_a::<JsString>()
                    || child_val.is_a::<JsNumber>()
                {
                    let content = escape_text_content_for_browser(scope, child_val);
                    if previous_was_text_node {
                        html.push_str("<!-- -->");
                    }
                    html.push_str(content.as_str());
                    current_is_text_node = true;
                } else if child_val.is_a::<JsObject>() {
                    render_cost += render_type(
                        html,
                        scope,
                        child.to_raw(),
                        static_markup,
                        current_is_text_node,
                        level+1
                    );
                } else {
                    println!(">>> child={}", to_string(scope, &child));
                    panic!("Invalid child type");
                }
            }
        };
        html.push_str(footer.as_str());
    } else {
        if JsValue::from_raw(component).as_value(scope).is_a::<JsArray>() {
            println!("component is a JsArray");
        }
        println!(">>> component={}", to_string(scope, &JsObject::from_raw(component)));
        println!(">>> type={}", to_string(scope, &type_obj));
        println!(">>> props={}", to_string(scope, &props));
        let children = get_children(scope, props.to_raw());
        for child in children {
            println!(">>> child={}", to_string(scope, &child));
        }
        panic!("Invalid component type");
    }
    render_cost
}

#[derive(Eq, PartialEq)]
pub enum ReadSize {
    Infinity,
    // How many bytes to read
    Size(usize),
}

impl PartialOrd for ReadSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match (self, other) {
                (&ReadSize::Infinity, &ReadSize::Infinity) => Ordering::Equal,
                (&ReadSize::Infinity, &ReadSize::Size(_)) => Ordering::Greater,
                (&ReadSize::Size(_), &ReadSize::Infinity) => Ordering::Less,
                (&ReadSize::Size(a), &ReadSize::Size(b)) => a.cmp(&b),
            }
        )
    }
}

pub struct Frame {
    dom_namespace: &'static str,
    children: Vec<JsValue>,
    child_index: u32,
    context: JsObject,
    footer: String,
}

pub struct DomServerRenderer<'a> {
    pub call: Call<'a>,
    stack: Vec<Frame>,
    exhausted: bool,
    current_select_value: JsValue,
    previous_was_text_node: bool,
    static_markup: bool,
}

impl<'a> DomServerRenderer<'a> {
    pub fn new(call: Call<'a>, static_markup: bool) -> Self {
        let top_frame = Frame {
            dom_namespace: HTML_NAMESPACE,
            children: vec![],
            child_index: 0,
            context: JsObject::from_raw(
                JsObject::new(call.scope)
                    .deref()
                    .to_raw()
            ),
            footer: String::new(),
        };
        let stack = vec![top_frame];
        let exhausted = false;
        let current_select_value = *JsNull::new().as_value(call.scope).deref();
        let previous_was_text_node = false;
        DomServerRenderer{
            call,
            stack,
            exhausted,
            current_select_value,
            previous_was_text_node,
            static_markup,
        }
    }

    pub fn read(&mut self, size: ReadSize) -> Option<String> {
        let now = Instant::now();
        let mut html = String::new();
        let component = self.call
            .arguments
            .get(self.call.scope, 0)
            .unwrap()
            .to_raw();
        let render_cost = render_type(
            &mut html, self.call.scope, component, self.static_markup, false, 0
        );
        // println!("[Render cost]: {}", duration_str(render_cost));
        // println!("[Render total cost]: {}", duration_str(now.elapsed()));
        Some(html)
    }

    pub fn render(
        &mut self,
        child: &JsValue,
        context: &JsObject,
        parent_namespace: &str
    ) -> String {
        "".to_owned()
    }

    pub fn render_DOM(
        &mut self,
        element: &JsValue,
        context: &JsObject,
        parent_namespace: &'static str
    ) -> String {
        let tag = JsObject
            ::from_raw(element.to_raw()).get(self.call.scope, "type")
            .unwrap()
            .deref()
            .to_string(self.call.scope)
            .unwrap()
            .deref()
            .value()
            .to_lowercase();
        let namespace = match parent_namespace {
            HTML_NAMESPACE => get_intrinsic_namespace(tag.as_str()),
            _ => parent_namespace
        };
        validate_dangerous_tag(tag.as_str());
        let props = get_obj(self.call.scope, element.to_raw(), "props");
        match tag.as_str() {
            "input" => {},
            "textarea" => {},
            "select" => {},
            "option" => {},
            _ => {},
        };
        "".to_owned()
    }
}
