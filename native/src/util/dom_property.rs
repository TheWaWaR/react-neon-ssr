use std::collections::{HashSet, HashMap};

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
use neon::mem::Handle;

use util::html_dom_property_config as html_config;
use util::svg_dom_property_config as svg_config;

// react/packages/react-dom/src/shared/DOMProperty.js
pub const MUST_USE_PROPERTY: u32 = 0x1;
pub const HAS_BOOLEAN_VALUE: u32 = 0x4;
pub const HAS_NUMERIC_VALUE: u32 = 0x8;
pub const HAS_POSITIVE_NUMERIC_VALUE: u32 = 0x10 | 0x8;
pub const HAS_OVERLOADED_BOOLEAN_VALUE: u32 = 0x20;
pub const HAS_STRING_BOOLEAN_VALUE: u32 = 0x40;

/* eslint-disable max-len */
pub const ATTRIBUTE_NAME_START_CHAR: &str =
    ":A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD";
/* eslint-enable max-len */
pub const ID_ATTRIBUTE_NAME: &str = "data-reactid";
pub const ROOT_ATTRIBUTE_NAME: &str = "data-reactroot";

lazy_static! {
    pub static ref ATTRIBUTE_NAME_CHAR: String = format!(
        "{}{}", ATTRIBUTE_NAME_START_CHAR, "\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040"
    );
    pub static ref RESERVED_PROPS: HashSet<&'static str> = hashset!{
        "children",
        "dangerouslySetInnerHTML",
        "defaultValue",
        "defaultChecked",
        "innerHTML",
        "suppressContentEditableWarning",
        "suppressHydrationWarning",
        "style",
    };
    pub static ref PROPERTIES: HashMap<String, PropertyInfo> = {
        let mut m = HashMap::new();
        inject_dom_property_config(
            &mut m,
            &DOMPropertyConfig {
                properties: html_config::PROPERTIES.clone(),
                dom_attribute_namespaces: HashMap::new(),
                dom_attribute_names: html_config::DOM_ATTRIBUTE_NAMES.clone(),
            }
        );
        inject_dom_property_config(
            &mut m,
            &DOMPropertyConfig {
                properties: svg_config::PROPERTIES.clone(),
                dom_attribute_namespaces: svg_config::DOM_ATTRIBUTE_NAMESPACES.clone(),
                dom_attribute_names: svg_config::DOM_ATTRIBUTE_NAMES.clone(),
            }
        );
        m
    };
}

pub struct PropertyInfo {
    pub attribute_name: String,
    pub attribute_namespace: Option<&'static str>,
    pub property_name: String,
    // -- NOTE: Seems only used by: client/DOMPropertyOperations.js
    // --       Which is client side!
    // mutationMethod: Option<String>,

    pub must_use_property: bool,
    pub has_boolean_value: bool,
    pub has_numeric_value: bool,
    pub has_positive_numeric_value: bool,
    pub has_overloaded_boolean_value: bool,
    pub has_string_boolean_value: bool,
}

struct DOMPropertyConfig {
    properties: HashMap<String, u32>,
    dom_attribute_namespaces: HashMap<&'static str, &'static str>,
    dom_attribute_names: HashMap<String, &'static str>,

    // -- NOTE: Seems only used by: client/DOMPropertyOperations.js
    // --       Which is client side!
    // DOMMutationMethods: (),
}

fn inject_dom_property_config(
    props: &mut HashMap<String, PropertyInfo>,
    config: &DOMPropertyConfig,
) {
    for (prop_name, prop_config) in &(config.properties) {
        debug_assert!(
            !props.contains_key(prop_name.as_str()),
            r##"injectDOMPropertyConfig(...): You're trying to inject DOM property
 {} which has already been injected. You may be accidentally
 injecting the same DOM property config twice, or you may be
 injecting two configs that have conflicting property names."##,
            prop_name,
        );
        let attribute_name = config.dom_attribute_names
            .get(prop_name)
            .map(|s| String::from(*s))
            .unwrap_or_else(|| prop_name.to_lowercase());
        let attribute_namespace = config.dom_attribute_namespaces
            .get(prop_name.as_str())
            .map(|s| *s);
        let info = PropertyInfo {
            attribute_name: attribute_name,
            attribute_namespace: attribute_namespace,
            property_name: prop_name.to_string(),

            must_use_property: check_mask(*prop_config, MUST_USE_PROPERTY),
            has_boolean_value: check_mask(*prop_config, HAS_BOOLEAN_VALUE),
            has_numeric_value: check_mask(*prop_config, HAS_NUMERIC_VALUE),
            has_positive_numeric_value: check_mask(
                *prop_config, HAS_POSITIVE_NUMERIC_VALUE),
            has_overloaded_boolean_value: check_mask(
                *prop_config, HAS_OVERLOADED_BOOLEAN_VALUE),
            has_string_boolean_value: check_mask(
                *prop_config, HAS_STRING_BOOLEAN_VALUE),
        };
        debug_assert!(
            info.has_boolean_value as u16 +
                info.has_numeric_value as u16 +
                info.has_overloaded_boolean_value as u16 <= 1,
            r##"DOMProperty: Value can be one of boolean, overloaded boolean, or
numeric value, but not a combination: {}"##,
            prop_name,
        );
        props.insert(prop_name.to_string(), info);
    }

}

pub fn check_mask(value: u32, bitmask: u32) -> bool {
    (value & bitmask) == bitmask
}

pub fn should_set_attribute(name: &str, value: Handle<JsValue>) -> bool {
    if is_reserved_prop(name) {
        return false;
    }
    if name.len() > 2 {
        let mut chars = name.chars();
        let char0 = chars.next().unwrap();
        let char1 = chars.next().unwrap();
        if (char0 == 'o' || char0 == 'O') &&
            (char1 == 'n' || char1 == 'N')
        {
            return false;
        }
    }
    if value.is_a::<JsNull>() {
        return true;
    }
    if value.is_a::<JsBoolean>() {
        return should_attribute_accept_boolean_value(name);
    }
    if value.is_a::<JsUndefined>() ||
        value.is_a::<JsNumber>() ||
        value.is_a::<JsString>() ||
        value.is_a::<JsObject>()
    {
        return true;
    }
    false
}

pub fn get_property_info(name: &str) -> Option<&PropertyInfo> {
    PROPERTIES.get(name)
}

pub fn should_attribute_accept_boolean_value(name: &str) -> bool {
    if is_reserved_prop(name) {
        return true;
    }
    if let Some(info) = get_property_info(name) {
        return info.has_boolean_value
            || info.has_string_boolean_value
            || info.has_overloaded_boolean_value
    }
    let prefix: String = name.to_lowercase().chars().take(5).collect();
    prefix.as_str() == "data-" || prefix.as_str() == "aria-"
}

pub fn is_reserved_prop(name: &str) -> bool {
    RESERVED_PROPS.contains(name)
}
