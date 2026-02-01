mod button;
mod crime;
mod map;
mod reader;
mod item;
mod inventoryy;

use crate::{inventoryy::Inventory, item::Item, button::button::Button};

use turbo::*;
use turbo::text_box::TextBox;
use turbo::time::tick;

#[turbo::game]
struct GameState {
    // Add fields here
    pub inven: inventoryy::Inventory,
    pub reader: reader::Reader,
    uiButtons:[Button; 3],

    pub day: i32,
    pub currTime: usize,
    pub invenHold: bool,
    pub invenCheck: usize,
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
            invenHold: false,
            invenCheck: 0,
        }
    }
    pub fn update(&mut self) {
        // This is where your main game loop code goes
        // The stuff in this block will run ~60x per sec
        let mut select: (f32,f32) = (0.0,0.0);
        let m = pointer::world();
        let(mx, my) = m.xy();
        let x = mx as f32;
        let y = my as f32;

        
        rect!(x = 350, y = 90, w = 220, h = 220, color = 0x0000ffff, rotation = 45);

        let mut yOffset = 180.0;
        for n in 0..self.inven.inven.len() {
            select = self.inven.invenB[n].check(select);
            
            if n == 0 {
                self.inven.invenB[n].hitbox.0 = 70.0;
                self.inven.invenB[n].hitbox.1 = 180.0;
            } else if n % 2 == 0 {
                yOffset += 60.0;
                self.inven.invenB[n].hitbox.0 = 70.0;
                self.inven.invenB[n].hitbox.1 = yOffset; 
            } else {
                self.inven.invenB[n].hitbox.0 = 130.0;
                self.inven.invenB[n].hitbox.1 = yOffset;
                 
            }
            for k in 0..self.reader.currMap.interactable.len() {
                let hitbox1 = self.reader.currMap.interactable[k].hitbox;
                let hitbox2 = self.inven.invenB[n].hitbox;
                if self.reader.currMap.interactable[k].hover(hitbox1, x, y) &&
                self.inven.invenB[n].hover(hitbox2, x, y) && m.just_released() &&
                self.reader.currMap.interactable[k].text == "" && self.invenHold{
                    self.inven.inven.remove(k);

                } 
            }

            
            let origin = self.inven.invenB[n].hitbox;

            if self.inven.invenB[n].action && !self.invenHold {
                self.invenHold = true;
                self.invenCheck = n
            }

            if self.inven.invenB[n].action && self.invenHold && self.invenCheck != n
            || !self.invenHold
            || !self.inven.invenB[n].action && self.invenHold {
                self.inven.invenB[n].action = false;
                self.inven.invenB[n].hold = false;
                self.inven.invenB[n].hitbox = origin;
            }
            if self.inven.invenB[n].hold {
                self.inven.invenB[n].hitbox.0 = x - (self.inven.invenB[n].hitbox.2/2.0);
                self.inven.invenB[n].hitbox.1 = y - (self.inven.invenB[n].hitbox.3/2.0);
            }

            if m.just_released() {
                self.inven.invenB[n].action = false;
                self.inven.invenB[n].hold = false;
                self.invenHold = false;
                self.invenCheck = 0;
            }
            self.inven.invenB[n].tempDraw("name");
            text!("{}", self.inven.inven[n].name; x = self.inven.invenB[n].hitbox.0,y = self.inven.invenB[n].hitbox.0,);
        }

        for n in 0..self.reader.currMap.interactable.len() {
            select = self.reader.currMap.interactable[n].check(select);
            if self.reader.currMap.interactable[n].action {
                if self.reader.currMap.interactable[n].text == "" {
                    self.reader.currMap.interactable[n].action = false;
                    break;
                } else {
                    self.inven.inven.push(self.reader.currMap.items[n].clone());
                    self.inven.setButton();
                    self.reader.currMap.interactable[n].text = "".to_string();
                    self.reader.currMap.interactable[n].action = false;
                             
                    }
            }
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
            // for l in 0..self.reader.currCrime.availPos.len() {
            //     if self.reader.currCrime.availPos[l] == self.currTime {
            //         self.reader.currMap.interactable[l].tempDraw("no");
            //     }
            //     text!("{}", self.reader.currCrime.availPos[l]; x = 10, y = 50, font = "TENPIXELS", color = 0x2d1e1eff);
            // }
            text!("{}", self.reader.currCrime.availPos[0]; x = 10, y = 50, font = "TENPIXELS", color = 0x2d1e1eff);
        }

    }
}