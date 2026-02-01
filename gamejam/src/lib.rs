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
    uiButtons:[Button; 4],

    pub day: i32,
    pub currTime: usize,
    pub invenHold: bool,
    pub invenCheck: usize,
    pub dayCheck: Vec<usize>,
    pub alibiTime: Vec<usize>,
    pub alibiOrder: Vec<String>,
    pub removed: bool,
}

impl GameState {
    pub fn new() -> Self {
        // initialize your game state
        Self { 
            inven: inventoryy::Inventory::new(),
            reader: reader::Reader::new(),
            uiButtons: [
                Button::new("nextCrime", (367.0, 445.0, 154.0, 28.0), false),
                Button::new("leftTime", (23.0, 115.0, 39.0, 26.0), false),
                Button::new("rightTime",(118.0, 115.0, 39.0, 26.0), false),
                Button::new("nextCrime", (550.0, 445.0, 154.0, 28.0), false),
            ],

            day: 0,
            currTime: 0,
            invenHold: false,
            invenCheck: 0,
            dayCheck: Vec::new(),
            alibiTime: Vec::new(),
            alibiOrder: Vec::new(),
            removed: false,
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

        
        //rect!(x = 350, y = 90, w = 220, h = 220, color = 0x0000ffff, rotation = 45);
        
        sprite!("ui", x = 0, y = 0);
        sprite!("bg", x = 0, y = 0);

        let mut yOffset = 180.0;
        
        //checking inventory
        for n in 0..self.inven.inven.len() {
            select = self.inven.invenB[n].check(select);
            
            //if its the very first item, give it the first spot
            if n == 0 {
                self.inven.invenB[n].hitbox.0 = 70.0;
                self.inven.invenB[n].hitbox.1 = 180.0;
            }
            //if its an even number, the x position stays the same, but the y position goes lower 
            else if n % 2 == 0 {
                yOffset += 60.0;
                self.inven.invenB[n].hitbox.0 = 70.0;
                self.inven.invenB[n].hitbox.1 = yOffset; 
            }
            //otherwise if its an odd number (the right hand side of the grid), move the x pos over 
            else {
                self.inven.invenB[n].hitbox.0 = 130.0;
                self.inven.invenB[n].hitbox.1 = yOffset;
                 
            }

            //trying to check if the inventory item and the interactable space on the map is overlapping
            //still working on this
            for k in 0..self.reader.currMap.interactable.len() {
                let hitbox1 = self.reader.currMap.interactable[k].hitbox;
                text!("hi", x = 0, y = 10, color = 0xffffffff);
                if self.reader.currMap.interactable[k].hover(hitbox1, x, y) &&
                self.inven.invenB[n].hold && m.just_released() && self.reader.currMap.interactable[k].text == ""
                && self.reader.currCrime.answerKey[k] == self.inven.inven[n].name &&
                self.reader.currCrime.answerTime[k] == self.currTime
                {
                    text!("hi", x = 20, y = 10, color = 0xffffffff);
                    self.alibiOrder[k] = self.inven.inven[n].name.clone();
                    self.alibiTime[k] = self.currTime;
                    self.inven.inven.remove(n);
                    self.inven.invenB.remove(n);
                    self.removed = true;
                } 
                text!("bye", x = 50, y = 10, color = 0xffffffff);
            }

            if self.removed {
                self.removed = false;
                break;
            }
            //keeping the origin in case i need to make the item return to its original spot
            let origin = self.inven.invenB[n].hitbox;

            //if there is an action being done to the item (ie. mouse press/mouse hold) and you already aren't holding an item,
            //make it so that you are definitively holding that item only
            if self.inven.invenB[n].action && !self.invenHold {
                self.invenHold = true;
                self.invenCheck = n
            }

            //if an action is being done on an item and you are holding something, but it isn't that specific item
            //or if you aren't holding an item
            //or if an action isn't being done on an item and you are holding something
            //make the action on that specific item false along with the hold check and make it go back
            //to its original spot
            if self.inven.invenB[n].action && self.invenHold && self.invenCheck != n
            || !self.invenHold
            || !self.inven.invenB[n].action && self.invenHold {
                self.inven.invenB[n].action = false;
                self.inven.invenB[n].hold = false;
                self.inven.invenB[n].hitbox = origin;
            }

            //if the hold check is true, make the item stick to the mouse regardless if you are
            //in the hitbox or not
            if self.inven.invenB[n].hold {
                self.inven.invenB[n].hitbox.0 = x - (self.inven.invenB[n].hitbox.2/2.0);
                self.inven.invenB[n].hitbox.1 = y - (self.inven.invenB[n].hitbox.3/2.0);
            }

            //once you release the mouse, the hold, action, invenHold, and inventory id check all becomes false
            if m.just_released() {
                self.inven.invenB[n].action = false;
                self.inven.invenB[n].hold = false;
                self.invenHold = false;
                self.invenCheck = 0;
            }

            //temp draw the item
            self.inven.invenB[n].tempDraw("name");
            text!("{}", self.inven.inven[n].name; x = self.inven.invenB[n].hitbox.0,y = self.inven.invenB[n].hitbox.0,);
        }


        //checking all the interactable items on the map
        for n in 0..self.reader.currMap.interactable.len() {
            select = self.reader.currMap.interactable[n].check(select);

            if self.day > 0 {
                for l in 0..self.reader.currCrime.availPos.len() {
                    if self.reader.currCrime.availPos[l] != self.currTime {
                        self.reader.currMap.interactable[l].action = false;
                    }
                }
             }
            //if the current item on the map is being interacted with
            if self.reader.currMap.interactable[n].action {
                //if the item has nothing there (why its an empty string), then nothing will happen
                if self.reader.currMap.interactable[n].text == "" {
                    self.reader.currMap.interactable[n].action = false;
                    break;
                } 
                //otherwise, assuming its an item, push the item into the inventory
                //load the setButton, so that the inventory shows the new uploaded item
                //and make the interactable spot empty.
                //if you want to make a check for a dialogue one, just use an if statement to check the text of that spot
                else {
                    self.inven.inven.push(self.reader.currMap.items[n].clone());
                    self.inven.setButton();
                    self.reader.currMap.interactable[n].text = "".to_string();
                    self.reader.currMap.interactable[n].action = false;
                             
                    }
            }
            //temp draw
            
        }
        
        //checking all UIButtons
        for n in 0..self.uiButtons.len() {
            select = self.uiButtons[n].check(select);

            //if there was an action done on an UIButton
            if self.uiButtons[n].action {
                //check which specific one
                match n {
                    //this one is the serve alibi, proceeds day and level data
                    0 => {
                        self.day += 1;
                        self.reader.changeLevel(self.day);
                        self.dayCheck = self.reader.currCrime.availPos.clone();
                        self.alibiOrder = vec!["".to_string(); self.reader.currCrime.answerPos.len()];
                        self.alibiTime = vec![0; self.reader.currCrime.answerTime.len()];
                        self.uiButtons[n].action = false;
                    }
                    //left button on the time, if the current time is 0 (the earliest time),
                    //then it does nothing, otherwise, it'll go down to the time earlier than current
                    1 => {
                        if self.currTime == 0 {
                            self.uiButtons[n].action = false;
                            break;
                        } else {
                            self.currTime -= 1;
                            self.uiButtons[n].action = false;
                        }
                        
                    }
                    //right button on the time, if the current time is the max, does nothing
                    //otherwise, it'll increment to the later time
                    2 => {
                        if self.currTime == self.reader.currMap.timeP.len()-1{
                            self.uiButtons[n].action = false;
                            break;
                        } else {
                            self.currTime += 1;
                            self.uiButtons[n].action = false;
                        }
                    }
                    3 => {
                        self.reader.currCrime.alibiCheck(self.alibiOrder.clone(), self.alibiTime.clone());
                        self.uiButtons[n].action = false;
                    }
                    _=> {}
                }
            }
            //just drawing
            if n == 0 {
                self.uiButtons[n].draw();
            } else {
                self.uiButtons[n].draw();
            }
        }

        //bunch of text print for suspect ID and report
        text!("Suspect: {}", self.reader.currCrime.name; x = 735, y = 153, font = "TENPIXELS", color = 0x2d1e1eff);
        let mut yOffset = 180;
        for n in 0..self.reader.currCrime.detail.len() {
            text_box!(self.reader.currCrime.detail[n].as_str(), x = 735, y = yOffset, w = 133, h = 50, font = "TENPIXELS", color = 0x2d1e1eff);
            yOffset += 60;
        }
        
        
        //only prints if the day is greater than 0 because some data doesn't exist till day 1
        if self.day > 0 {
            if self.reader.currMap.timeP[self.currTime] > 0{
                text!("{} AM", self.reader.currMap.timeP[self.currTime]; x = 110, y = 120, font = "TENPIXELS", color = 0x2d1e1eff);
            } else {
                text!("{} PM", self.reader.currMap.timeP[self.currTime]*(-1); x = 110, y = 120, font = "TENPIXELS", color = 0x2d1e1eff);
            }
            let mut yoff = 50;

            //prints out the interactable items at the specific time only
            for l in 0..self.reader.currCrime.availPos.len() {
                if self.reader.currCrime.availPos[l] == self.currTime {
                    self.reader.currMap.interactable[l].tempDraw("no");
                    text!("{}", self.reader.currMap.interactable[l].text; x = 100, y = yoff, font = "TENPIXELS", color = 0x2d1e1eff);
                } else {
                    self.reader.currMap.interactable[l].tempDraw("hi");
                }
                text!("{}", self.reader.currCrime.availPos[l]; x = 10, y = yoff, font = "TENPIXELS", color = 0x2d1e1eff);
                yoff += 10;
            }
            text!("{}", self.reader.currCrime.availPos[0]; x = 10, y = 50, font = "TENPIXELS", color = 0x2d1e1eff);
            for n in 0..self.alibiOrder.len() {
                text!("{:?}", self.alibiOrder[n]; x = 20, y = yoff + 300, font = "TENPIXELS", color = 0x2d1e1eff);
                yoff += 20;
                text!("{}", self.alibiTime[n]; x = 10, y = yoff + 300, font = "TENPIXELS", color = 0x2d1e1eff);
            }
        }

    }
}