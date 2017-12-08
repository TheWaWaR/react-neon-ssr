use std::collections::HashMap;

use std::ops::Index;
use regex::{Regex, Captures};

use util::dom_property::HAS_STRING_BOOLEAN_VALUE;

pub const NS_XLINK: &str = "http://www.w3.org/1999/xlink";
pub const NS_XML: &str = "http://www.w3.org/XML/1998/namespace";

lazy_static! {
    pub static ref NS: HashMap<&'static str, &'static str> = hashmap! {
        "xlink" => NS_XLINK,
        "xml"   => NS_XML,
    };

    /**
     * This is a list of all SVG attributes that need special casing,
     * namespacing, or boolean value assignment.
     *
     * When adding attributes to this list, be sure to also add them to
     * the `possibleStandardNames` module to ensure casing and incorrect
     * name warnings.
     *
     * SVG Attributes List:
     * https://www.w3.org/TR/SVG/attindex.html
     * SMIL Spec:
     * https://www.w3.org/TR/smil
     */
    pub static ref ATTRS: Vec<&'static str> = vec![
        "accent-height",
        "alignment-baseline",
        "arabic-form",
        "baseline-shift",
        "cap-height",
        "clip-path",
        "clip-rule",
        "color-interpolation",
        "color-interpolation-filters",
        "color-profile",
        "color-rendering",
        "dominant-baseline",
        "enable-background",
        "fill-opacity",
        "fill-rule",
        "flood-color",
        "flood-opacity",
        "font-family",
        "font-size",
        "font-size-adjust",
        "font-stretch",
        "font-style",
        "font-variant",
        "font-weight",
        "glyph-name",
        "glyph-orientation-horizontal",
        "glyph-orientation-vertical",
        "horiz-adv-x",
        "horiz-origin-x",
        "image-rendering",
        "letter-spacing",
        "lighting-color",
        "marker-end",
        "marker-mid",
        "marker-start",
        "overline-position",
        "overline-thickness",
        "paint-order",
        "panose-1",
        "pointer-events",
        "rendering-intent",
        "shape-rendering",
        "stop-color",
        "stop-opacity",
        "strikethrough-position",
        "strikethrough-thickness",
        "stroke-dasharray",
        "stroke-dashoffset",
        "stroke-linecap",
        "stroke-linejoin",
        "stroke-miterlimit",
        "stroke-opacity",
        "stroke-width",
        "text-anchor",
        "text-decoration",
        "text-rendering",
        "underline-position",
        "underline-thickness",
        "unicode-bidi",
        "unicode-range",
        "units-per-em",
        "v-alphabetic",
        "v-hanging",
        "v-ideographic",
        "v-mathematical",
        "vector-effect",
        "vert-adv-y",
        "vert-origin-x",
        "vert-origin-y",
        "word-spacing",
        "writing-mode",
        "x-height",
        "xlink:actuate",
        "xlink:arcrole",
        "xlink:href",
        "xlink:role",
        "xlink:show",
        "xlink:title",
        "xlink:type",
        "xml:base",
        "xmlns:xlink",
        "xml:lang",
        "xml:space",
    ];

    static ref CAMELIZE: Regex = Regex::new("[-:]([a-z])").unwrap();

    static ref CAMELIZED_ATTRS: HashMap<&'static str, String> = {
        let mut m = HashMap::new();
        for original in ATTRS.iter() {
            let react_name = CAMELIZE.replace_all(original, |caps: &Captures| {
                caps.index(1).to_uppercase()
            }).to_string();
            m.insert(*original, react_name);
        }
        m
    };

    pub static ref PROPERTIES: HashMap<String, u32> = {
        let mut m = hashmap! {
            "autoReverse".to_string()               => HAS_STRING_BOOLEAN_VALUE,
            "externalResourcesRequired".to_string() => HAS_STRING_BOOLEAN_VALUE,
            "preserveAlpha".to_string()             => HAS_STRING_BOOLEAN_VALUE,
        };
        for react_name in CAMELIZED_ATTRS.values() {
            m.insert(react_name.clone(), 0);
        }
        m
    };

    pub static ref DOM_ATTRIBUTE_NAMES: HashMap<String, &'static str> = {
        let mut m = hashmap! {
            "autoReverse".to_string()               => "autoReverse",
            "externalResourcesRequired".to_string() => "externalResourcesRequired",
            "preserveAlpha".to_string()             => "preserveAlpha",
        };
        for (original, react_name) in CAMELIZED_ATTRS.iter() {
            m.insert(react_name.clone(), original);
        }
        m
    };

    pub static ref DOM_ATTRIBUTE_NAMESPACES: HashMap<&'static str, &'static str> = hashmap! {
        "xlinkActuate" => NS_XLINK,
        "xlinkArcrole" => NS_XLINK,
        "xlinkHref"    => NS_XLINK,
        "xlinkRole"    => NS_XLINK,
        "xlinkShow"    => NS_XLINK,
        "xlinkTitle"   => NS_XLINK,
        "xlinkType"    => NS_XLINK,
        "xmlBase"      => NS_XML,
        "xmlLang"      => NS_XML,
        "xmlSpace"     => NS_XML,
    };
}
