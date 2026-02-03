use crate::item::Item;
use crate::button::button::Button;

#[turbo::serialize]
//Inventory and the invetory button (so you can interact with it)
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
            match self.inven[n].name.as_str() {
                "money" => {self.inven[n].name = "money_inven".to_string();}
                "flower" => {self.inven[n].name = "flower_inven".to_string();}
                "phone" => {self.inven[n].name = "phone_inven".to_string();}
                _=> {}
            }
            self.invenB.push(Button::new(&self.inven[n].name, (34.0, 180.0, 51.0, 51.0), false));
        }
    }

    
}