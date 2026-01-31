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
        let mut yOffset = 180.0;
        for n in 0..self.inven.len() {
            if n == 0 {
                self.invenB.push(Button::new(&self.inven[n].name, (70.0, 180.0, 45.0, 45.0), false));
            } else if n % 2 == 0 {
                yOffset += 60.0;
                self.invenB.push(Button::new(&self.inven[n].name, (70.0, yOffset, 45.0, 45.0), false));
            } else {
                
                self.invenB.push(Button::new(&self.inven[n].name, (130.0, yOffset, 45.0, 45.0), false));
            }
        }
    }
}