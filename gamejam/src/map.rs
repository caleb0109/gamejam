use crate::item::Item;
use crate::button::button::Button;

#[turbo::serialize]

//the amount of time points you can access
//items that are available in the map
//interactable points in the map
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