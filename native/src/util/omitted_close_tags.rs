use std::collections::HashSet;

// react/packages/react-dom/src/shared/omittedCloseTags.js
lazy_static! {
    pub static ref OMITTED_CLOSE_TAGS: HashSet<&'static str> = hashset!{
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

    pub static ref VOID_ELEMENT_TAGS: HashSet<&'static str> = {
        let mut m = OMITTED_CLOSE_TAGS.clone();
        m.insert("menuitem");
        m
    };
}
