
pub mod dangerous_style_value;
pub mod css_property;
pub mod css_property_operations;
pub mod html_dom_property_config;
pub mod svg_dom_property_config;
pub mod dom_property;
pub mod omitted_close_tags;
pub mod void_element_tags;
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


use std::time::Duration;
use std::ops::{Deref, Index};
use std::sync::{Arc, Mutex};

use regex::{Regex, Captures};
use neon_runtime::raw::Local;
use neon::scope::{RootScope, Scope};
use neon::mem::{Handle, Managed};
use neon::js::{Value, Object, Variant};
use neon::js::{
    JsFunction,
    JsObject,
    JsNumber,
    JsValue,
    JsNull,
    JsUndefined,
    JsString,
};


const REACT_ELEMENT_TYPE_NUMBER: u32 = 0xeac7;

lazy_static! {
    static ref UPPERCASE_PATTERN: Regex = Regex::new("([A-Z])").unwrap();
    static ref MS_PATTERN: Regex = Regex::new("^ms-").unwrap();
    static ref REACT_ELEMENT_TYPE: Arc<Mutex<Option<String>>> = {
        Arc::new(Mutex::new(None))
    };
}

pub fn duration_str(value: Duration) -> String {
    let seconds = value.as_secs();
    let ns = value.subsec_nanos();
    let ms = ns as f64 / 1000_000.0;
    let mut s = String::new();
    if seconds > 0 {
        s.push_str(format!("{}s", seconds).as_str());
    }
    if ms > 0.0 {
        s.push_str(format!("{:.3}ms", ms).as_str());
    }
    s
}

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

pub fn to_string<T: Value>(scope: &mut RootScope, obj: &T) -> String {
    obj.to_string(scope)
        .unwrap()
        .deref()
        .value()
}

// The Symbol used to tag the ReactElement type. If there is no native Symbol
// nor polyfill, then a plain number is used for performance.
// var REACT_ELEMENT_TYPE =
//     (typeof Symbol === 'function' && Symbol.for && Symbol.for('react.element')) ||
//     0xeac7;

/**
 * Verifies the object is a ReactElement.
 * See https://reactjs.org/docs/react-api.html#isvalidelement
 * @param {?object} object
 * @return {boolean} True if `object` is a valid component.
 * @final
 */
fn is_valid_element(scope: &mut RootScope, obj: Handle<JsObject>) -> bool {
    // function isValidElement(object) {
    //     return (
    //         typeof object === 'object' &&
    //             object !== null &&
    //             object.$$typeof === REACT_ELEMENT_TYPE
    //     );
    // }
    let mut react_element_type = REACT_ELEMENT_TYPE.lock().unwrap();
    if react_element_type.is_none() {
        let mut value = String::new();
        let global = scope.global();
        let symbol = global
            .get(scope, "Symbol")
            .unwrap();
        if symbol.is_a::<JsFunction>() {
            let symbol_for = get_fn(scope, symbol.to_raw(), "for");
            let this = symbol_for.as_value(scope);
            let argument = JsString::new(scope, "react.element").unwrap();
            value = symbol_for
                .call(scope, this, vec![argument])
                .unwrap()
                .to_string(scope)
                .unwrap()
                .value();
        }
        react_element_type.get_or_insert(value);
    }
    if obj.is_a::<JsObject>() && !obj.is_a::<JsNull>() {
        match JsObject
            ::from_raw(obj.to_raw())
            .get(scope, "$$typeof")
            .unwrap()
            .variant()
        {
            Variant::Number(number) => {
                number.value() as u32 == REACT_ELEMENT_TYPE_NUMBER
            },
            Variant::String(s) => {
                Some(s.value()) == *react_element_type
            }
            _ => false
        }
    } else {
        false
    }
}


pub fn not(value: Handle<JsValue>) -> bool {
    // [MDN]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_Operators
    //   * undefined
    //   * null
    //   * NaN
    //   * 0
    //   * Empty String ("")

    // ::FIXME: May have bug
    match value.variant() {
        Variant::Null(_) | Variant::Undefined(_) => true,
        Variant::Number(v) => v.value() == 0.0,
        Variant::String(v) => v.value().len() == 0,
        _ => false
    }
}

pub fn is_nan(value: Handle<JsValue>) -> bool {
    !value.is_a::<JsNumber>()
}

/**
 * Hyphenates a camelcased string, for example:
 *
 *   > hyphenate('backgroundColor')
 *   < "background-color"
 *
 * For CSS style names, use `hyphenateStyleName` instead which works properly
 * with all vendor prefixes, including `ms`.
 *
 * @param {string} string
 * @return {string}
 */
pub fn hyphenate(value: &str) -> String {
    // function hyphenate(string) {
    //     return string.replace(_uppercasePattern, '-$1').toLowerCase();
    // }
    UPPERCASE_PATTERN.replace_all(value, |caps: &Captures| {
        format!("-{}", caps.index(0))
    }).to_lowercase()
}

/**
 * Hyphenates a camelcased CSS property name, for example:
 *
 *   > hyphenateStyleName('backgroundColor')
 *   < "background-color"
 *   > hyphenateStyleName('MozTransition')
 *   < "-moz-transition"
 *   > hyphenateStyleName('msTransition')
 *   < "-ms-transition"
 *
 * As Modernizr suggests (http://modernizr.com/docs/#prefixed), an `ms` prefix
 * is converted to `-ms-`.
 *
 * @param {string} string
 * @return {string}
 */
pub fn hyphenate_style_name(value: &str) -> String {
    // function hyphenateStyleName(string) {
    //     return hyphenate(string).replace(msPattern, '-ms-');
    // }
    MS_PATTERN.replace_all(hyphenate(value).as_str(), |_: &Captures| {
        "-ms-".to_string()
    }).to_string()
}

