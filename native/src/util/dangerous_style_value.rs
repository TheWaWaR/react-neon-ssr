// DONE
use neon::mem::Handle;
use neon::js::{
    JsBoolean,
    JsNull,
    JsNumber,
    JsString,
    JsValue,
};

use util::css_property::IS_UNITLESS_NUMBER;

/**
 * Convert a value into the proper css writable value. The style name `name`
 * should be logical (no hyphens), as specified
 * in `CSSProperty.isUnitlessNumber`.
 *
 * @param {string} name CSS property name such as `topMargin`.
 * @param {*} value CSS property value such as `10px`.
 * @return {string} Normalized style value with dimensions applied.
 */

pub fn dangerous_style_value(
    name: &str,
    value: Handle<JsValue>,
    is_custom_property: bool,
) -> String {
    // Note that we've removed escapeTextForBrowser() calls here since the
    // whole string will be escaped when the attribute is injected into
    // the markup. If you provide unsafe user data here they can inject
    // arbitrary CSS which may be problematic (I couldn't repro this):
    // https://www.owasp.org/index.php/XSS_Filter_Evasion_Cheat_Sheet
    // http://www.thespanner.co.uk/2007/11/26/ultimate-xss-css-injection/
    // This is not an XSS hole but instead a potential CSS injection issue
    // which has lead to a greater discussion about how we're going to
    // trust URLs moving forward. See #2115901
    if value.is_a::<JsNull>()
        || value.is_a::<JsBoolean>()
    {
        return "".to_string();
    }
    let text = value.downcast::<JsString>().map(|v| v.value());
    if text == Some("".to_string()) {
        return "".to_string();
    }

    let number = value.downcast::<JsNumber>().map(|v| v.value() as u32);
    if !is_custom_property
        && number.is_some()
        && number != Some(0)
        && !IS_UNITLESS_NUMBER.contains(name)
    {
        // Presumes implicit 'px' suffix for unitless numbesr
        return format!("{}px", number.unwrap());
    }
    text.map(|s| s.trim().to_owned())
        .unwrap_or_else(|| "".to_string())
}
