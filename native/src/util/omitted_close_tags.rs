use std::collections::HashMap;

// react/packages/react-dom/src/shared/omittedCloseTags.js
lazy_static! {
    pub static ref omittedCloseTags: HashMap<&'static str, bool> = {
        let mut m = HashMap::new();
        m.insert("area", true);
        m.insert("base", true);
        m.insert("br", true);
        m.insert("col", true);
        m.insert("embed", true);
        m.insert("hr", true);
        m.insert("img", true);
        m.insert("input", true);
        m.insert("keygen", true);
        m.insert("link", true);
        m.insert("meta", true);
        m.insert("param", true);
        m.insert("source", true);
        m.insert("track", true);
        m.insert("wbr", true);
        m
    };

    pub static ref voidElementTags: HashMap<&'static str, bool> = {
        let mut m = omittedCloseTags.clone();
        m.insert("menuitem", true);
        m
    };
}
