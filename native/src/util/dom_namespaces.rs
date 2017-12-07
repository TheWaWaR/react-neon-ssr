
use std::collections::HashMap;

// react/packages/react-dom/src/shared/DOMNamespaces.js
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const MATH_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";
const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

lazy_static! {
    static ref Namespaces: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("html", HTML_NAMESPACE);
        m.insert("mathml", MATH_NAMESPACE);
        m.insert("svg", SVG_NAMESPACE);
        m
    };
}
