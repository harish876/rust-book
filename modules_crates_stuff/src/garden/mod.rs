
#[derive(Debug)]
pub(crate) struct Vegetable {
    color: String,
    size: String,
}

impl Vegetable {
    pub fn new(color: String, size: String) -> Self {
        Self { color, size }
    }
}

pub(crate) fn print_garden() {}
