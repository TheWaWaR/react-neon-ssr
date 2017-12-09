
use std::ops::Index;

use regex::{Regex, Captures};
use neon::scope::{RootScope};
use neon::js::{Value};
use neon::js::{
    JsValue,
    JsBoolean,
    JsNumber,
    JsString,
};
use neon::mem::Handle;

lazy_static! {
    static ref REGEX: Regex = Regex::new("[\"<>'&]").unwrap();
}

pub fn escape_html(text: &str) -> String {
    REGEX.replace_all(text, |caps: &Captures| {
        match caps.index(0) {
            "\"" => "&quot;",
            "&" => "&amp;",
            "'" => "&#x27;",
            "<" => "&lt;",
            ">" => "&gt;",
            _ => unreachable!()
        }.to_owned()
    }).to_string()
}

pub fn escape_text_content_for_browser(
    obj: Handle<JsValue>, scope: &mut RootScope,
) -> String {
    let text = obj
        .to_string(scope)
        .unwrap()
        .value();
    if obj.is_a::<JsBoolean>() || obj.is_a::<JsNumber>() {
        text
    } else {
        escape_html(text.as_str())
    }
}
