mod button;
mod crime;
mod map;
mod reader;
mod item;
mod inventoryy;

use crate::{inventoryy::Inventory, item::Item, button::button::Button};

use turbo::*;

#[turbo::game]
struct GameState {
    // Add fields here
    pub inven: inventoryy::Inventory,
    pub reader: reader::Reader,
    uiButtons:[Button; 3],

    pub day: i32,
    pub currTime: usize,
}
impl GameState {
    pub fn new() -> Self {
        // initialize your game state
        Self { 
            inven: inventoryy::Inventory::new(),
            reader: reader::Reader::new(),
            uiButtons: [
                Button::new("nextCrime", (350.0,400.0, 100.0, 40.0), false),
                Button::new("leftTime", (80.0, 120.0, 20.0,10.0), false),
                Button::new("rightTime",(160.0, 120.0, 20.0,10.0), false),
            ],

            day: 0,
            currTime: 0,
        }
    }
    pub fn update(&mut self) {
        // This is where your main game loop code goes
        // The stuff in this block will run ~60x per sec
        let mut select: (f32,f32) = (0.0,0.0);
        rect!(x = 350, y = 90, w = 220, h = 220, color = 0x0000ffff, rotation = 45);
        let newInven = vec![
            Item::new("Dummy", "Fake dummy of person"),
            Item::new("Key", "Key that opens a door to where?"),
            Item::new("hi", "hi"),
            Item::new("hi","hi"),
            Item::new("hi", "hi"),
            Item::new("hi","hi"),
        ];
        self.inven.inven = newInven;
        self.inven.setButton();
        for n in 0..self.inven.inven.len() {
            select = self.inven.invenB[n].check(select);
            self.inven.invenB[n].tempDraw("name");
        }

        for n in 0..self.reader.currMap.interactable.len() {
            select = self.reader.currMap.interactable[n].check(select);
            self.reader.currMap.interactable[n].tempDraw("hi");
        }
        for n in 0..self.uiButtons.len() {
            select = self.uiButtons[n].check(select);
            if self.uiButtons[n].action {
                match n {
                    0 => {
                        self.day += 1;
                        self.reader.changeLevel(self.day);
                        self.uiButtons[n].action = false;
                    }
                    1 => {
                        if self.currTime == 0 {
                            self.uiButtons[n].action = false;
                            break;
                        } else {
                            self.currTime -= 1;
                            self.uiButtons[n].action = false;
                        }
                        
                    }
                    2 => {
                        if self.currTime == self.reader.currMap.timeP.len()-1{
                            self.uiButtons[n].action = false;
                            break;
                        } else {
                            self.currTime += 1;
                            self.uiButtons[n].action = false;
                        }
                    }
                    _=> {}
                }
            }
            if n == 0 {
                self.uiButtons[n].tempDraw("day");
            } else {
                self.uiButtons[n].tempDraw("hi");
            }
        }

        text!("Suspect: {}", self.reader.currCrime.name; x = 600, y = 100, font = "TENPIXELS", color = 0x2d1e1eff);
        let mut yOffset = 120;
        for n in 0..self.reader.currCrime.detail.len() {
            text!(self.reader.currCrime.detail[n].as_str(), x = 600, y = yOffset, font = "TENPIXELS", color = 0x2d1e1eff);
            yOffset += 20;
        }
        
        
        if self.day > 0 {
            if self.reader.currMap.timeP[self.currTime] > 0{
                text!("{} AM", self.reader.currMap.timeP[self.currTime]; x = 110, y = 120, font = "TENPIXELS", color = 0x2d1e1eff);
            } else {
                text!("{} PM", self.reader.currMap.timeP[self.currTime]*(-1); x = 110, y = 120, font = "TENPIXELS", color = 0x2d1e1eff);
            }
        }

    }
}