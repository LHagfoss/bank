use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Card {
    pub uuid: Uuid,
    pub name: String,
}

impl Card {
    pub fn new(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
        }
    }

    pub fn change_name(&mut self, new_name: &str) -> Result<(), &'static str> {
        if new_name.is_empty() {
            return Err("Card name cannot be empty");
        }
        self.name = new_name.to_string();
        Ok(())
    }
}
