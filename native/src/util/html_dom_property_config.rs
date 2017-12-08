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
    pub static ref Properties: HashMap<&'static str, u32> = hashmap! {
        "allowFullScreen" => HAS_BOOLEAN_VALUE,
        "autoFocus"       => HAS_STRING_BOOLEAN_VALUE,
        "async"           => HAS_BOOLEAN_VALUE,
        "autoPlay"        => HAS_BOOLEAN_VALUE,
        "capture"         => HAS_BOOLEAN_VALUE,
        "checked"         => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "cols"            => HAS_POSITIVE_NUMERIC_VALUE,
        "contentEditable" => HAS_STRING_BOOLEAN_VALUE,
        "controls"        => HAS_BOOLEAN_VALUE,
        "default"         => HAS_BOOLEAN_VALUE,
        "defer"           => HAS_BOOLEAN_VALUE,
        "disabled"        => HAS_BOOLEAN_VALUE,
        "download"        => HAS_OVERLOADED_BOOLEAN_VALUE,
        "draggable"       => HAS_STRING_BOOLEAN_VALUE,
        "formNoValidate"  => HAS_BOOLEAN_VALUE,
        "hidden"          => HAS_BOOLEAN_VALUE,
        "loop"            => HAS_BOOLEAN_VALUE,
        "multiple"        => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "muted"           => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "noValidate"      => HAS_BOOLEAN_VALUE,
        "open"            => HAS_BOOLEAN_VALUE,
        "playsInline"     => HAS_BOOLEAN_VALUE,
        "readOnly"        => HAS_BOOLEAN_VALUE,
        "required"        => HAS_BOOLEAN_VALUE,
        "reversed"        => HAS_BOOLEAN_VALUE,
        "rows"            => HAS_POSITIVE_NUMERIC_VALUE,
        "rowSpan"         => HAS_NUMERIC_VALUE,
        "scoped"          => HAS_BOOLEAN_VALUE,
        "seamless"        => HAS_BOOLEAN_VALUE,
        "selected"        => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "size"            => HAS_POSITIVE_NUMERIC_VALUE,
        "start"           => HAS_NUMERIC_VALUE,
        "span"            => HAS_POSITIVE_NUMERIC_VALUE,
        "spellCheck"      => HAS_STRING_BOOLEAN_VALUE,
        "style"           => 0,
        "tabIndex"        => 0,
        "itemScope"       => HAS_BOOLEAN_VALUE,
        "acceptCharset"   => 0,
        "className"       => 0,
        "htmlFor"         => 0,
        "httpEquiv"       => 0,
        "value"           => HAS_STRING_BOOLEAN_VALUE,
    };

    pub static ref DOMAttributeNames:  HashMap<&'static str, &'static str> = hashmap! {
        "acceptCharset" => "accept-charset",
        "className"     => "class",
        "htmlFor"       => "for",
        "httpEquiv"     => "http-equiv",
    };
}
