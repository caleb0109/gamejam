use crate::item::Item;

#[turbo::serialize]

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
}