use neon::scope::RootScope;
use neon::mem::Handle;
use neon::js::JsValue;

/**
 * Escapes attribute value to prevent scripting attacks.
 *
 * @param {*} value Value to escape.
 * @return {string} An escaped string.
 */

use util::escape_text_content_for_browser::escape_text_content_for_browser;

pub fn quote_attribute_value_for_browser(
    scope: &mut RootScope, obj: Handle<JsValue>
) -> String {
    format!("\"{}\"", escape_text_content_for_browser(scope, obj))
}
