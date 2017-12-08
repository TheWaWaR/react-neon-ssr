
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

fn get_intrinsic_namespace(t: &str) -> &'static str {
    match t {
        "svg" => SVG_NAMESPACE,
        "math" => MATH_NAMESPACE,
        _ => HTML_NAMESPACE,
    }
}

fn get_child_namespace<'a>(parent: Option<&'a str>, t: &str) -> &'a str {
    if parent.is_none() || parent == Some(HTML_NAMESPACE) {
        return get_intrinsic_namespace(t);
    }
    if parent == Some(SVG_NAMESPACE) && t == "foreignObject" {
        return HTML_NAMESPACE;
    }
    parent.unwrap()
}
