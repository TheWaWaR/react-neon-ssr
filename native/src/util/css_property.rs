
use std::collections::HashSet;

// react/packages/react-dom/src/shared/CSSProperty.js

/**
 * Support style names that may come passed in prefixed by adding permutations
 * of vendor prefixes.
 */
pub const CSSPrefixes: [&str; 4] = ["Webkit", "ms", "Moz", "O"];
lazy_static! {
    /**
     * CSS properties which accept numbers but are not in units of "px".
     */
    pub static ref isUnitlessNumber: HashSet<&'static str> = hashset!{
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
    };
}
