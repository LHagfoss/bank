use serde::Serialize;
use uuid::Uuid;

use crate::{card::Card, person::Person};

#[derive(Debug, Serialize)]
pub struct Account {
    pub uuid: Uuid,
    pub name: String,
    pub person: Person,
    pub balance: f32,
    pub card: Option<Card>,
}

impl Account {
    pub fn new(name: &str, person: Person) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            person,
            balance: 0.0,
            card: None,
        }
    }

    pub fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f32) -> Result<(), &'static str> {
        if self.balance < amount {
            return Err("Insufficient balance");
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn issue_card(&mut self, card_name: &str) -> Result<(), &'static str> {
        if self.card.is_some() {
            return Err("You already have a card");
        }
        if card_name.is_empty() {
            return Err("Card name cannot be empty");
        }
        self.card = Some(Card::new(card_name));
        Ok(())
    }
}
