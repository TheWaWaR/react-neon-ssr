
use std::collections::HashMap;

// react/packages/react-dom/src/shared/CSSProperty.js

/**
 * Support style names that may come passed in prefixed by adding permutations
 * of vendor prefixes.
 */
const CSSPrefixes: [&str; 4] = ["Webkit", "ms", "Moz", "O"];
lazy_static! {
    /**
     * CSS properties which accept numbers but are not in units of "px".
     */
    static ref isUnitlessNumber: HashMap<&'static str, bool> = {
        let mut m = HashMap::new();
        m.insert("animationIterationCount", true);
        m.insert("borderImageOutset", true);
        m.insert("borderImageSlice", true);
        m.insert("borderImageWidth", true);
        m.insert("boxFlex", true);
        m.insert("boxFlexGroup", true);
        m.insert("boxOrdinalGroup", true);
        m.insert("columnCount", true);
        m.insert("columns", true);
        m.insert("flex", true);
        m.insert("flexGrow", true);
        m.insert("flexPositive", true);
        m.insert("flexShrink", true);
        m.insert("flexNegative", true);
        m.insert("flexOrder", true);
        m.insert("gridRow", true);
        m.insert("gridRowEnd", true);
        m.insert("gridRowSpan", true);
        m.insert("gridRowStart", true);
        m.insert("gridColumn", true);
        m.insert("gridColumnEnd", true);
        m.insert("gridColumnSpan", true);
        m.insert("gridColumnStart", true);
        m.insert("fontWeight", true);
        m.insert("lineClamp", true);
        m.insert("lineHeight", true);
        m.insert("opacity", true);
        m.insert("order", true);
        m.insert("orphans", true);
        m.insert("tabSize", true);
        m.insert("widows", true);
        m.insert("zIndex", true);
        m.insert("zoom", true);

        // SVG-related properties
        m.insert("fillOpacity", true);
        m.insert("floodOpacity", true);
        m.insert("stopOpacity", true);
        m.insert("strokeDasharray", true);
        m.insert("strokeDashoffset", true);
        m.insert("strokeMiterlimit", true);
        m.insert("strokeOpacity", true);
        m.insert("strokeWidth", true);
        m
    };
}
