use std::collections::HashSet;

// react/packages/react-dom/src/shared/DOMProperty.js
pub const MUST_USE_PROPERTY: u32 = 0x1;
pub const HAS_BOOLEAN_VALUE: u32 = 0x4;
pub const HAS_NUMERIC_VALUE: u32 = 0x8;
pub const HAS_POSITIVE_NUMERIC_VALUE: u32 = 0x10 | 0x8;
pub const HAS_OVERLOADED_BOOLEAN_VALUE: u32 = 0x20;
pub const HAS_STRING_BOOLEAN_VALUE: u32 = 0x40;

/* eslint-disable max-len */
pub const ATTRIBUTE_NAME_START_CHAR: &str =
    ":A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD";
/* eslint-enable max-len */
pub const ID_ATTRIBUTE_NAME: &str = "data-reactid";
pub const ROOT_ATTRIBUTE_NAME: &str = "data-reactroot";

lazy_static! {
    pub static ref ATTRIBUTE_NAME_CHAR: String = format!(
        "{}{}", ATTRIBUTE_NAME_START_CHAR, "\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040"
    );
    pub static ref RESERVED_PROPS: HashSet<&'static str> = hashset!{
        "children",
        "dangerouslySetInnerHTML",
        "defaultValue",
        "defaultChecked",
        "innerHTML",
        "suppressContentEditableWarning",
        "suppressHydrationWarning",
        "style",
    };
}
