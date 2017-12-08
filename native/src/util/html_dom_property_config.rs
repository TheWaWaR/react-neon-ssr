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
    pub static ref PROPERTIES: HashMap<String, u32> = hashmap! {
        "allowFullScreen".to_string() => HAS_BOOLEAN_VALUE,
        "autoFocus".to_string()       => HAS_STRING_BOOLEAN_VALUE,
        "async".to_string()           => HAS_BOOLEAN_VALUE,
        "autoPlay".to_string()        => HAS_BOOLEAN_VALUE,
        "capture".to_string()         => HAS_BOOLEAN_VALUE,
        "checked".to_string()         => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "cols".to_string()            => HAS_POSITIVE_NUMERIC_VALUE,
        "contentEditable".to_string() => HAS_STRING_BOOLEAN_VALUE,
        "controls".to_string()        => HAS_BOOLEAN_VALUE,
        "default".to_string()         => HAS_BOOLEAN_VALUE,
        "defer".to_string()           => HAS_BOOLEAN_VALUE,
        "disabled".to_string()        => HAS_BOOLEAN_VALUE,
        "download".to_string()        => HAS_OVERLOADED_BOOLEAN_VALUE,
        "draggable".to_string()       => HAS_STRING_BOOLEAN_VALUE,
        "formNoValidate".to_string()  => HAS_BOOLEAN_VALUE,
        "hidden".to_string()          => HAS_BOOLEAN_VALUE,
        "loop".to_string()            => HAS_BOOLEAN_VALUE,
        "multiple".to_string()        => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "muted".to_string()           => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "noValidate".to_string()      => HAS_BOOLEAN_VALUE,
        "open".to_string()            => HAS_BOOLEAN_VALUE,
        "playsInline".to_string()     => HAS_BOOLEAN_VALUE,
        "readOnly".to_string()        => HAS_BOOLEAN_VALUE,
        "required".to_string()        => HAS_BOOLEAN_VALUE,
        "reversed".to_string()        => HAS_BOOLEAN_VALUE,
        "rows".to_string()            => HAS_POSITIVE_NUMERIC_VALUE,
        "rowSpan".to_string()         => HAS_NUMERIC_VALUE,
        "scoped".to_string()          => HAS_BOOLEAN_VALUE,
        "seamless".to_string()        => HAS_BOOLEAN_VALUE,
        "selected".to_string()        => MUST_USE_PROPERTY | HAS_BOOLEAN_VALUE,
        "size".to_string()            => HAS_POSITIVE_NUMERIC_VALUE,
        "start".to_string()           => HAS_NUMERIC_VALUE,
        "span".to_string()            => HAS_POSITIVE_NUMERIC_VALUE,
        "spellCheck".to_string()      => HAS_STRING_BOOLEAN_VALUE,
        "style".to_string()           => 0,
        "tabIndex".to_string()        => 0,
        "itemScope".to_string()       => HAS_BOOLEAN_VALUE,
        "acceptCharset".to_string()   => 0,
        "className".to_string()       => 0,
        "htmlFor".to_string()         => 0,
        "httpEquiv".to_string()       => 0,
        "value".to_string()           => HAS_STRING_BOOLEAN_VALUE,
    };

    pub static ref DOM_ATTRIBUTE_NAMES: HashMap<String, &'static str> = hashmap! {
        "acceptCharset".to_string() => "accept-charset",
        "className".to_string()     => "class",
        "htmlFor".to_string()       => "for",
        "httpEquiv".to_string()     => "http-equiv",
    };
}
