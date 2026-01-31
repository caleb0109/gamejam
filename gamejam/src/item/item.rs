use turbo::*;

#[turbo::serialize]

pub struct Item {
    pub name: String,
    pub description: String,
}

impl Item {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

