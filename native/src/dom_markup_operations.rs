use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use regex::Regex;
use neon::scope::RootScope;
use neon::mem::Handle;
use neon::js::{
    JsValue,
    JsNull,
    JsBoolean,
    JsNumber,
};


use util::dom_property::{
    ATTRIBUTE_NAME_CHAR,
    ATTRIBUTE_NAME_START_CHAR,
    ID_ATTRIBUTE_NAME,
    ROOT_ATTRIBUTE_NAME,
    get_property_info,
    should_attribute_accept_boolean_value,
    should_set_attribute,
};
use util::dom_property::PropertyInfo;
use util::{
    not,
    is_nan,
    quote_attribute_value_for_browser
};


lazy_static! {
    static ref MARKUP_FOR_ROOT: String = {
        format!("{}=\"\"", ROOT_ATTRIBUTE_NAME)
    };
    static ref VALID_ATTRIBUTE_NAME_REGEX: Regex = {
        let pattern = format!(
            "^[{}][{}]*$",
            ATTRIBUTE_NAME_START_CHAR,
            *ATTRIBUTE_NAME_CHAR
        );
        Regex::new(pattern.as_str()).unwrap()
    };
    static ref ILLEGAL_ATTRIBUTE_NAME_CACHE: Arc<Mutex<HashSet<String>>> = {
        Arc::new(Mutex::new(HashSet::new()))
    };
    static ref VALIDATED_ATTRIBUTE_NAME_CACHE: Arc<Mutex<HashSet<String>>> = {
        Arc::new(Mutex::new(HashSet::new()))
    };
}

fn is_attribute_name_safe(attribute_name: &str) -> bool {
    let mut validated_cache = VALIDATED_ATTRIBUTE_NAME_CACHE.lock().unwrap();
    if validated_cache.contains(attribute_name) {
        return true;
    }
    let mut illegal_cache = ILLEGAL_ATTRIBUTE_NAME_CACHE.lock().unwrap();
    if illegal_cache.contains(attribute_name) {
        return true;
    }
    if VALID_ATTRIBUTE_NAME_REGEX.is_match(attribute_name) {
        validated_cache.insert(attribute_name.to_string());
        return true;
    }
    // TODO:
    // if (__DEV__) {
    //     warning(false, 'Invalid attribute name: `%s`', attributeName);
    // }
    illegal_cache.insert(attribute_name.to_string());
    false
}

// shouldIgnoreValue() is currently duplicated in DOMPropertyOperations.
// TODO: Find a better place for this.
fn should_ignore_value(
    info: &PropertyInfo,
    value: Handle<JsValue>
) -> bool {
    value.is_a::<JsNull>()
        || (info.has_boolean_value && not(value))
        || (info.has_numeric_value && is_nan(value))
        || (info.has_positive_numeric_value
            && value.downcast::<JsNumber>().unwrap().value() < 1.0)
        || (info.has_overloaded_boolean_value
            && value.downcast::<JsBoolean>().unwrap().value() == false)
}

/**
 * Creates markup for the ID property.
 *
 * @param {string} id Unescaped ID.
 * @return {string} Markup string.
 */
pub fn create_markup_for_id(
    scope: &mut RootScope,
    id: Handle<JsValue>
) -> String {
    let quoted_id = quote_attribute_value_for_browser(scope, id);
    format!("{}={}", ID_ATTRIBUTE_NAME, quoted_id)
}

pub fn create_markup_for_root() -> String {
    MARKUP_FOR_ROOT.clone()
}

/**
 * Creates markup for a property.
 *
 * @param {string} name
 * @param {*} value
 * @return {?string} Markup string, or null if the property was invalid.
 */
pub fn create_markup_for_property(
    scope: &mut RootScope,
    name: &str,
    value: Handle<JsValue>
) -> Option<String> {
    // TODO:
    let property_info = get_property_info(name);
    if let Some(info) = property_info {
        if should_ignore_value(info, value) {
            return Some("".to_string());
        }
        let is_boolean = value.is_a::<JsBoolean>();
        if info.has_boolean_value || (
            info.has_overloaded_boolean_value
                && is_boolean
                && value.downcast::<JsBoolean>().unwrap().value() == true
        ) {
            return Some(format!("{}=\"\"", info.attribute_name));
        } else if !is_boolean || should_attribute_accept_boolean_value(name) {
            let quoted_value = quote_attribute_value_for_browser(scope, value);
            return Some(format!("{}={}", info.attribute_name, quoted_value));
        }
    } else if should_set_attribute(name, value) {
        if value.is_a::<JsNull>() {
            return Some("".to_string());
        }
        let quoted_value = quote_attribute_value_for_browser(scope, value);
        return Some(format!("{}={}", name, quoted_value));
    }
    None
}

/**
 * Creates markup for a custom property.
 *
 * @param {string} name
 * @param {*} value
 * @return {string} Markup string, or empty string if the property was invalid.
 */
pub fn create_markup_for_custom_attribute(
    scope: &mut RootScope,
    name: &str,
    value: Handle<JsValue>,
) -> String {
    // TODO:
    if !is_attribute_name_safe(name) || value.is_a::<JsNull>() {
        "".to_string()
    } else {
        let quoted_value = quote_attribute_value_for_browser(scope, value);
        format!("{}={}", name, quoted_value)
    }
}
