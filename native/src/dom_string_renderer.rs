
use partial_renderer::{ReadSize, DomServerRenderer};


pub fn render_to_string(element: ()) -> Vec<u8> {
    DomServerRenderer::new(vec![element], false)
        .read(ReadSize::Infinity)
}

pub fn render_to_static_markup(element: ()) -> Vec<u8> {
    DomServerRenderer::new(vec![element], true)
        .read(ReadSize::Infinity)
}
