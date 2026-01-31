
#[turbo::serialize]

pub struct Crime {
    pub name: String,
    pub detail: Vec<String>,
    pub alibi: String,
}

impl Crime {
    pub fn new(name: &str, detail: Vec<String>, alibi: &str) -> Self {
        Self {
            name: name.to_string(),
            detail: detail,
            alibi: alibi.to_string(),
        }
    }
}