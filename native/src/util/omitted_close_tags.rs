use std::collections::HashSet;

// react/packages/react-dom/src/shared/omittedCloseTags.js
lazy_static! {
    pub static ref omittedCloseTags: HashSet<&'static str> = hashset!{
        "area",
        "base",
        "br",
        "col",
        "embed",
        "hr",
        "img",
        "input",
        "keygen",
        "link",
        "meta",
        "param",
        "source",
        "track",
        "wbr",
    };

    pub static ref voidElementTags: HashSet<&'static str> = {
        let mut m = omittedCloseTags.clone();
        m.insert("menuitem");
        m
    };
}
