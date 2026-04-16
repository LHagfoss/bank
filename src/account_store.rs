use std::collections::HashMap;

use serde::Serialize;
use uuid::Uuid;

use crate::account::Account;

#[derive(Debug, Serialize)]
pub struct AccountStore {
    pub inner: HashMap<Uuid, Account>,
}

impl AccountStore {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn list_all(&self) -> Vec<&Account> {
        self.inner.values().collect()
    }

    pub fn find_by_id(&self, account_id: Uuid) -> Option<&Account> {
        self.inner.get(&account_id)
    }

    pub fn find_by_phone(&self, phone_number: i32) -> Option<&Account> {
        self.inner
            .values()
            .find(|a| a.person.phone_number == Some(phone_number))
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Account> {
        self.inner.values().find(|a| a.person.name == name)
    }
}
