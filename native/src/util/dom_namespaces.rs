
use std::collections::HashMap;

// react/packages/react-dom/src/shared/DOMNamespaces.js
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub const MATH_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

lazy_static! {
    pub static ref Namespaces: HashMap<&'static str, &'static str> = hashmap! {
        "html"   => HTML_NAMESPACE,
        "mathml" => MATH_NAMESPACE,
        "svg"    => SVG_NAMESPACE,
    };
}
