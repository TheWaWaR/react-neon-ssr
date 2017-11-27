

pub struct DomServerRenderer {
    children: Vec<()>,
    static_markup: bool,
}

pub enum ReadSize {
    Infinity,
    // How many bytes to read
    Size(usize),
}

impl DomServerRenderer {
    pub fn new(children: Vec<()>, static_markup: bool) -> Self {
        DomServerRenderer{children, static_markup}
    }

    pub fn read(&mut self, size: ReadSize) -> Vec<u8> {
        Vec::new()
    }
}
