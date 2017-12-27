use std::collections::HashSet;

use util::omitted_close_tags::OMITTED_CLOSE_TAGS;

lazy_static! {
    pub static ref VOID_ELEMENT_TAGS: HashSet<&'static str> = {
        let mut s = OMITTED_CLOSE_TAGS.clone();
        s.insert("menuitem");
        s
    };
}
