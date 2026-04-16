use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub phone_number: Option<i32>,
}

impl Person {
    pub fn new(name: &str, age: i32, phone_number: Option<i32>) -> Self {
        Self {
            name: name.to_string(),
            age,
            phone_number,
        }
    }

    pub fn update_name(&mut self, new_name: &str) -> Result<(), &'static str> {
        if new_name.is_empty() {
            return Err("Person name cannot be empty");
        }

        if self.name == new_name {
            return Err("Person name is already set to that");
        }

        self.name = new_name.to_string();
        Ok(())
    }

    pub fn update_phone_number(&mut self, phone_number: i32) {
        self.phone_number = Some(phone_number);
    }
}
