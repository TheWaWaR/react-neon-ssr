
use std::collections::HashSet;

// react/packages/react-dom/src/shared/CSSProperty.js

/**
 * Support style names that may come passed in prefixed by adding permutations
 * of vendor prefixes.
 */
pub const CSS_PREFIXES: [&'static str; 4] = ["Webkit", "ms", "Moz", "O"];

lazy_static! {
    /**
     * CSS properties which accept numbers but are not in units of "px".
     */
    pub static ref IS_UNITLESS_NUMBER: HashSet<String> = {
        let names = vec![
            "animationIterationCount",
            "borderImageOutset",
            "borderImageSlice",
            "borderImageWidth",
            "boxFlex",
            "boxFlexGroup",
            "boxOrdinalGroup",
            "columnCount",
            "columns",
            "flex",
            "flexGrow",
            "flexPositive",
            "flexShrink",
            "flexNegative",
            "flexOrder",
            "gridRow",
            "gridRowEnd",
            "gridRowSpan",
            "gridRowStart",
            "gridColumn",
            "gridColumnEnd",
            "gridColumnSpan",
            "gridColumnStart",
            "fontWeight",
            "lineClamp",
            "lineHeight",
            "opacity",
            "order",
            "orphans",
            "tabSize",
            "widows",
            "zIndex",
            "zoom",

            // SVG-related properties
            "fillOpacity",
            "floodOpacity",
            "stopOpacity",
            "strokeDasharray",
            "strokeDashoffset",
            "strokeMiterlimit",
            "strokeOpacity",
            "strokeWidth",
        ];

        let mut s = HashSet::new();
        for name in names.iter() {
            s.insert(name.to_string());
            for prefix in CSS_PREFIXES.iter() {
                s.insert(prefix_key(prefix, name));
            }
        }
        s
    };
}

pub fn prefix_key(prefix: &str, key: &str) -> String {
    let mut c = key.chars();
    let new_key = match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect()
    };
    format!("{}{}", prefix, new_key)
}
