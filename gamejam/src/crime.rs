use crate::item::Item;

use turbo::*;
use turbo::text_box::TextBox;
use turbo::time::tick;
#[turbo::serialize]

//Name of the suspect
//report/detail of their crime
//the positions of the items the user are available to interact with
//the order of which the items should be in
//which time/hour the item should be in
//which position the item should be in
pub struct Crime {
    pub name: String,
    pub detail: Vec<String>,
    pub availPos: Vec<usize>,
    pub answerKey: Vec<String>,
    pub answerTime: Vec<usize>,
    pub answerPos: Vec<usize>,
}

impl Crime {
    pub fn new(name: &str, detail: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            detail: detail,
            availPos: Vec::new(),
            answerKey: Vec::new(),
            answerTime: Vec::new(),
            answerPos: Vec::new(),
        }
    }

    pub fn alibiCheck (&mut self, alibiItem: Vec<String>, alibiTime: Vec<usize>) {
        if self.answerKey == alibiItem && self.answerTime == alibiTime{
            text!("YAY",  x = 10, y =  300, font = "TENPIXELS", color = 0x2d1e1eff);
        }
        text!("BOOO",  x = 50, y =  300, font = "TENPIXELS", color = 0x2d1e1eff);
    }
}