
use std::collections::HashSet;

use neon::mem::Handle;
use neon::js::{JsValue, JsString};

lazy_static! {
    static ref TAG_NAMES: HashSet<&'static str> = hashset! {
        // These are reserved SVG and MathML elements.
        // We don't mind this whitelist too much because we expect it to never grow.
        // The alternative is to track the namespace in a few places which is convoluted.
        // https://w3c.github.io/webcomponents/spec/custom/#custom-elements-core-concepts
        "annotation-xml",
        "color-profile",
        "font-face",
        "font-face-src",
        "font-face-uri",
        "font-face-format",
        "font-face-name",
        "missing-glyph",
    };
}

pub fn is_customer_component(tag_name: &str, props: Handle<JsValue>) -> bool {
    if !tag_name.contains("-") {
        return props.is_a::<JsString>()
    }
    return !TAG_NAMES.contains(tag_name)
}
