
pub mod dangerous_style_value;
pub mod css_property;
pub mod css_property_operations;
pub mod html_dom_property_config;
pub mod svg_dom_property_config;
pub mod dom_property;
pub mod omitted_close_tags;
pub mod dom_namespaces;
pub mod html_node_type;
pub mod escape_text_content_for_browser;
pub mod quote_attribute_value_for_browser;
pub mod is_custom_component;

pub use self::dangerous_style_value::dangerous_style_value;
pub use self::is_custom_component::is_custom_component;
pub use self::css_property_operations::{
    create_dangerous_string_for_styles,
    set_value_for_styles,
};
pub use self::dom_namespaces::{
    get_intrinsic_namespace,
    get_child_namespace,
};
pub use self::css_property::prefix_key;
pub use self::dom_property::{
    check_mask,
    should_set_attribute,
    get_property_info,
    should_attribute_accept_boolean_value,
    is_reserved_prop,
};
pub use self::escape_text_content_for_browser::{
    escape_html,
    escape_text_content_for_browser
};
pub use self::quote_attribute_value_for_browser::{
    quote_attribute_value_for_browser,
};


use std::ops::Deref;

use neon_runtime::raw::Local;
use neon::scope::{RootScope};
use neon::mem::{Managed};
use neon::js::{Value, Object};
use neon::js::{
    JsFunction,
    JsObject,
};

pub fn get_raw(scope: &mut RootScope, obj: Local, key: &str) -> Local {
    JsObject::from_raw(obj)
        .get(scope, key)
        .unwrap()
        .deref()
        .to_raw()
}

pub fn get_obj(scope: &mut RootScope, obj: Local, key: &str) -> JsObject {
    JsObject::from_raw(get_raw(scope, obj, key))
}

pub fn get_fn(scope: &mut RootScope, obj: Local, key: &str) -> JsFunction {
    JsFunction::<JsObject>::from_raw(get_raw(scope, obj, key))
}

pub fn to_string<T: Value>(scope: &mut RootScope, obj: T) -> String {
    obj.to_string(scope)
        .unwrap()
        .deref()
        .value()
}
