use turbo::{text::Text, *};
static SCRIPT_PATH: &str = std::include_str!("script");
static DATA_PATH: &str = std::include_str!("data");
use crate::button::button::Button;
use crate::item::Item;
use crate::map::Map;
use crate::crime::Crime;


#[turbo::serialize]

pub struct Reader {
    pub sLines: Vec<String>,
    pub dLines: Vec<String>,
    pub current_line_s: usize,
    pub current_line_d: usize,
    pub currCrime: Crime,
    pub currMap: Map,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            sLines: SCRIPT_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            dLines: DATA_PATH.split("\r\n").map(|line| line.to_string()).collect(),
            current_line_s: 0,
            current_line_d: 0,
            currCrime: Crime::new("", Vec::new(), ""),
            currMap: Map::new(Vec::new(), Vec::new()),
        }
    }

    pub fn changeLevel(&mut self, day: i32) {
        match day {
            1 => {
                let n = self.sLines.iter().position(|line| line == "--crime1");
                let m = self.dLines.iter().position(|line| line == "--crime1");
                self.current_line_s = n.unwrap_or(0) + 1;
                self.current_line_d = m.unwrap_or(0) + 1;
                self.updateData();
            }
            2 => {
                let n = self.sLines.iter().position(|line| line == "--crime2");
                let m = self.dLines.iter().position(|line| line == "--crime2");
                self.current_line_s = n.unwrap_or(0) + 1;
                self.current_line_d = m.unwrap_or(0) + 1;
                self.updateData();
            }
            _=> {}
        }
    }

    pub fn updateData(&mut self) {
        let name = self.dLines[self.current_line_d].clone();
        self.currCrime.name = name;
        self.current_line_d += 1;
        let reportNum = self.dLines[self.current_line_d].parse::<usize>().unwrap();
        self.current_line_d += 1;

        for x in 0..reportNum {
            self.currCrime.detail.push(self.dLines[self.current_line_d].clone());
            self.current_line_d += 1;
            self.currMap.timeP.push(self.dLines[self.current_line_d].parse::<i32>().unwrap());
            self.current_line_d += 1;
        }

        let itemNum = self.dLines[self.current_line_d].parse::<usize>().unwrap();
        self.current_line_d += 1;

        for n in 0..itemNum {
            let iName = self.dLines[self.current_line_d].clone();
            self.current_line_d += 1;
            let iDesc = self.dLines[self.current_line_d].clone();
            self.current_line_d += 1;
            self.currMap.items.push(Item::new(&iName, &iDesc));

            let x = self.dLines[self.current_line_d].parse::<f32>().unwrap();
            self.current_line_d += 1;
            let y = self.dLines[self.current_line_d].parse::<f32>().unwrap();
            self.current_line_d += 1;
            self.currMap.interactable.push(Button::new(&iName, (x,y, 20.0,20.0), false));
        }

        

    }
}
