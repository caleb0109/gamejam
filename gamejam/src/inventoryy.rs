use crate::item::Item;
use crate::button::button::Button;

#[turbo::serialize]
pub struct Inventory {
    pub inven: Vec<Item>,
    pub invenB: Vec<Button>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            inven: Vec::new(),
            invenB: Vec::new(),
        }
    }

    pub fn setButton(&mut self) {
        for n in 0..self.inven.len() {
            self.invenB.push(Button::new("inven", (70.0, 180.0, 45.0, 45.0), false));
        }
    }
}