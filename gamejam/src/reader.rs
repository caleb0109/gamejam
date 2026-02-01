use std::string;

use turbo::{text::Text, *};
static SCRIPT_PATH: &str = std::include_str!("script");
static DATA_PATH: &str = std::include_str!("data");
use crate::button::button::Button;
use crate::item::Item;
use crate::map::Map;
use crate::crime::Crime;


#[turbo::serialize]

//script reader line
//data reader line
//current line for script
//current line for data
//current Crime we are at (current level, current day, same shit)
//current Map we are on      ^
pub struct Reader {
    pub sLines: Vec<String>,
    pub dLines: Vec<String>,
    pub current_line_s: usize,
    pub current_line_d: usize,
    pub currCrime: Crime,
    pub currMap: Map,
    pub speaking: bool,
    pub newSpeakSet: bool,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            sLines: SCRIPT_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            dLines: DATA_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            current_line_s: 0,
            current_line_d: 0,
            currCrime: Crime::new("", Vec::new()),
            currMap: Map::new(Vec::new(), Vec::new()),
            speaking: false,
            newSpeakSet: false,
        }
    }

    //similar to whats in Pocket_Pet and Boil & Bubble
    pub fn changeLevel(&mut self, day: i32) {
        match day {
            1 => {
                let n = self.sLines.iter().position(|line| line == "--crime1");
                let m = self.dLines.iter().position(|line| line == "--crime1");
                self.current_line_s = n.unwrap_or(0) + 1;
                self.current_line_d = m.unwrap_or(0) + 1;
                self.updateData();
                self.speaking = true;
            }
            2 => {
                let n = self.sLines.iter().position(|line| line == "--crime2");
                let m = self.dLines.iter().position(|line| line == "--crime2");
                self.current_line_s = n.unwrap_or(0) + 1;
                self.current_line_d = m.unwrap_or(0) + 1;
                self.updateData();
                self.speaking = true;
            }
            _=> {}
        }
    }

    //all for Data
    ///ORDER OF THE DATA
    /// SUSPECT NAME
    /// AMOUNT OF LINES FOR THE REPORT
    /// REPORT LINE
    /// REPORT TIME STAMP
    /// AMOUNT OF ITEMS IN THE MAP
    /// ITEM NAME
    /// ITEM DESCRIPTION
    /// ITEM X POSITION
    /// ITEM Y POSITION
    /// ANSWERKEY ORDER
    /// ANSWERTIME ORDER
    /// ANSWERPOSTION ORDER
    /// AVAILABLEPOSITION ORDER
    /// EXTRA INTERACTABLES AMOUNT
    /// EXTRA INTERACTABLE AND THEIR COORDINATES(ie. talkable npcs)
    pub fn updateData(&mut self) {
        //gets name
        let name = self.dLines[self.current_line_d].clone();
        self.currCrime.name = name;
        self.current_line_d += 1;
        //gets amount of report lines
        let reportNum = self.dLines[self.current_line_d].parse::<usize>().unwrap();
        self.current_line_d += 1;

        //for loop to get the report line and the time specified in report lines
        for x in 0..reportNum {
            self.currCrime.detail.push(self.dLines[self.current_line_d].clone());
            self.current_line_d += 1;
            self.currMap.timeP.push(self.dLines[self.current_line_d].parse::<i32>().unwrap());
            self.current_line_d += 1;
        }

        //amount of obtainable items in the map
        let itemNum = self.dLines[self.current_line_d].parse::<usize>().unwrap();
        self.current_line_d += 1;

        //for loop to get those items and the descriptions
        for n in 0..itemNum {
            let iName = self.dLines[self.current_line_d].clone();
            self.current_line_d += 1;
            let iDesc = self.dLines[self.current_line_d].clone();
            self.current_line_d += 1;
            self.currMap.items.push(Item::new(&iName, &iDesc));

            //gets the positions the items should be at as well
            let x = self.dLines[self.current_line_d].parse::<f32>().unwrap();
            self.current_line_d += 1;
            let y = self.dLines[self.current_line_d].parse::<f32>().unwrap();
            self.current_line_d += 1;
            self.currMap.interactable.push(Button::new(&iName, (x,y, 20.0,20.0), false));
        }
        
        let answerTotal = self.dLines[self.current_line_d].parse::<usize>().unwrap();
        self.current_line_d += 1;
        //for loop to get the answer key, time, and position
        for n in 0..answerTotal {
            self.currCrime.answerKey.push(self.dLines[self.current_line_d].clone());
            self.current_line_d += 1;
            self.currCrime.answerTime.push(self.dLines[self.current_line_d].parse::<usize>().unwrap());
            self.current_line_d +=1;
            self.currCrime.answerPos.push(self.dLines[self.current_line_d].parse::<usize>().unwrap());
            self.current_line_d += 1;
        }

        let extraInt = self.dLines[self.current_line_d].parse::<i32>().unwrap();
        self.currCrime.extraInt = extraInt as usize;
        self.current_line_d += 1;
        for n in 0..extraInt {
            let name = self.dLines[self.current_line_d].clone();
            self.current_line_d += 1;

            let x = self.dLines[self.current_line_d].parse::<f32>().unwrap();
            self.current_line_d += 1;
            let y = self.dLines[self.current_line_d].parse::<f32>().unwrap();
            self.current_line_d += 1;

            self.currMap.interactable.push(Button::new(&name, (x,y, 20.0,20.0), false));
        }

                //for loop to get the positions that are able to be interacted with during a specific time
        for n in 0..self.currMap.interactable.len() {
            self.currCrime.availPos.push(self.dLines[self.current_line_d].parse::<usize>().unwrap());
            self.current_line_d +=1;
        }


    }

    pub fn drawText(&mut self, start: &String) {        
        let text ;
        
        if !self.newSpeakSet {
            match start.as_str() {
                "--crime1" => {}
                "--crime2" => {}
                _=> {
                    let n = self.sLines.iter().position(|line| line == start);
                    self.current_line_s = n.unwrap_or(0) + 1;
                }
            }
            self.newSpeakSet = true;
        }
        
        text = &self.sLines[self.current_line_s];
        
            
        if self.speaking == true {
            //sprite!("speechbubble", x = 256, y= 114);
                text_box!{
                    text,
                    font = "FIVEPIXELS",
                    color = 0xfae3deff,
                    fixed = true,
                    width = 200,
                    height = 35,
                    x =  100,
                    y = 400,  
                    //end = time/5         
                }
            self.assessLine();
            
        }
    }

    pub fn assessLine(&mut self) {
        let m = pointer::world();
        if self.sLines[self.current_line_s] == "--end" {
            self.speaking = false;
            self.newSpeakSet = false;
        } else if m.just_released() {
            self.current_line_s += 1; 
        } 
    }

}
