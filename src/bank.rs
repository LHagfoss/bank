use std::collections::HashSet;

use serde::Serialize;
use uuid::Uuid;

use crate::{account::Account, account_store::AccountStore};

#[derive(Debug, Serialize)]
pub struct Bank {
    pub accounts: AccountStore,
    pub used_phone_numbers: HashSet<i32>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: AccountStore::new(),
            used_phone_numbers: HashSet::new(),
        }
    }

    pub fn open_account(&mut self, account: Account) -> Result<Uuid, &'static str> {
        if let Some(phone_number) = account.person.phone_number {
            if self.used_phone_numbers.contains(&phone_number) {
                return Err("Phone number is already in use by another account");
            }

            self.used_phone_numbers.insert(phone_number);
        }

        let id = account.uuid;
        self.accounts.inner.insert(id, account);
        Ok(id)
    }

    pub fn update_phone_number(
        &mut self,
        account_id: Uuid,
        new_phone_number: i32,
    ) -> Result<(), &'static str> {
        if self.used_phone_numbers.contains(&new_phone_number) {
            return Err("That phone number is already taken by someone else!");
        }

        let account = self
            .accounts
            .inner
            .get_mut(&account_id)
            .ok_or("Account not found")?;

        if let Some(old_phone) = account.person.phone_number {
            self.used_phone_numbers.remove(&old_phone);
        }

        self.used_phone_numbers.insert(new_phone_number);
        account.person.update_phone_number(new_phone_number);

        Ok(())
    }

    pub fn update_name(&mut self, account_id: Uuid, new_name: &str) -> Result<(), &'static str> {
        let account = self
            .accounts
            .inner
            .get_mut(&account_id)
            .ok_or("Account not found")?;

        match account.person.update_name(new_name) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn transfer(
        &mut self,
        from_id: Uuid,
        to_id: Uuid,
        amount: f32,
    ) -> Result<(), &'static str> {
        if from_id == to_id {
            return Err("Cannot transfer to the same account");
        }

        let sender = self
            .accounts
            .inner
            .get_mut(&from_id)
            .ok_or("Sender account not found")?;
        sender.withdraw(amount)?;

        let receiver = self
            .accounts
            .inner
            .get_mut(&to_id)
            .ok_or("Receiver account not found")?;
        receiver.deposit(amount);

        Ok(())
    }
}
