use crate::item::Item;
use crate::button::button::Button;

#[turbo::serialize]

pub struct Map {
    pub timeP: Vec<i32>,
    pub items: Vec<Item>,
    pub interactable: Vec<Button>,
}

impl Map {
    pub fn new(timeP: Vec<i32>, items: Vec<Item>) -> Self {
        Self {
            timeP: timeP,
            items: items,
            interactable: Vec::new(),
        }
    }

}