use std::collections::HashMap;

use util::dom_property::{
    MUST_USE_PROPERTY,
    HAS_BOOLEAN_VALUE,
    HAS_NUMERIC_VALUE,
    HAS_POSITIVE_NUMERIC_VALUE,
    HAS_OVERLOADED_BOOLEAN_VALUE,
    HAS_STRING_BOOLEAN_VALUE,
};

// react/packages/react-dom/src/shared/HTMLDOMPropertyConfig.js
lazy_static! {
    static ref Properties: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("allowFullScreen", HAS_BOOLEAN_VALUE);
        m.insert("autoFocus", HAS_STRING_BOOLEAN_VALUE);
        m.insert("async", HAS_BOOLEAN_VALUE);
        m.insert("autoPlay", HAS_BOOLEAN_VALUE);
        m.insert("capture", HAS_BOOLEAN_VALUE);
        m.insert("checked", MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE);
        m.insert("cols", HAS_POSITIVE_NUMERIC_VALUE);
        m.insert("contentEditable", HAS_STRING_BOOLEAN_VALUE);
        m.insert("controls", HAS_BOOLEAN_VALUE);
        m.insert("default", HAS_BOOLEAN_VALUE);
        m.insert("defer", HAS_BOOLEAN_VALUE);
        m.insert("disabled", HAS_BOOLEAN_VALUE);
        m.insert("download", HAS_OVERLOADED_BOOLEAN_VALUE);
        m.insert("draggable", HAS_STRING_BOOLEAN_VALUE);
        m.insert("formNoValidate", HAS_BOOLEAN_VALUE);
        m.insert("hidden", HAS_BOOLEAN_VALUE);
        m.insert("loop", HAS_BOOLEAN_VALUE);
        m.insert("multiple", MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE);
        m.insert("muted", MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE);
        m.insert("noValidate", HAS_BOOLEAN_VALUE);
        m.insert("open", HAS_BOOLEAN_VALUE);
        m.insert("playsInline", HAS_BOOLEAN_VALUE);
        m.insert("readOnly", HAS_BOOLEAN_VALUE);
        m.insert("required", HAS_BOOLEAN_VALUE);
        m.insert("reversed", HAS_BOOLEAN_VALUE);
        m.insert("rows", HAS_POSITIVE_NUMERIC_VALUE);
        m.insert("rowSpan", HAS_NUMERIC_VALUE);
        m.insert("scoped", HAS_BOOLEAN_VALUE);
        m.insert("seamless", HAS_BOOLEAN_VALUE);
        m.insert("selected", MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE);
        m.insert("size", HAS_POSITIVE_NUMERIC_VALUE);
        m.insert("start", HAS_NUMERIC_VALUE);
        m.insert("span", HAS_POSITIVE_NUMERIC_VALUE);
        m.insert("spellCheck", HAS_STRING_BOOLEAN_VALUE);
        m.insert("style", 0);
        m.insert("tabIndex", 0);
        m.insert("itemScope", HAS_BOOLEAN_VALUE);
        m.insert("acceptCharset", 0);
        m.insert("className", 0);
        m.insert("htmlFor", 0);
        m.insert("httpEquiv", 0);
        m.insert("value", HAS_STRING_BOOLEAN_VALUE);
        m
    };

    static ref DOMAttributeNames:  HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("acceptCharset", "accept-charset");
        m.insert("className", "class");
        m.insert("htmlFor", "for");
        m.insert("httpEquiv", "http-equiv");
        m
    };
}

