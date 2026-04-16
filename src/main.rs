use crate::{account::Account, bank::Bank};
use colored::*;
use lagos_logger::*;
use person::Person;

mod account;
mod account_store;
mod bank;
mod card;
mod person;

fn main() {
    let mut bank = Bank::new();
    log!("Bank initialized!");

    let person_one = Person::new("lucas", 18, Some(47669901));
    let person_two = Person::new("herman", 18, None);
    log!("New Person: \"Lucas\"");
    log!("New Person: \"Herman\"");

    let mut account_one = Account::new("account one", person_one);
    let mut account_two = Account::new("account two", person_two);
    log!("New Account: \"Account One\"");
    log!("New Account: \"Account Two\"");

    account_one.deposit(100.0);
    account_one.issue_card("lagos card").unwrap();
    log!("Deposited 100.0 into Accoutn One");
    log!("Issued \"Lagos Card\" for Account One");

    account_two.deposit(20.0);
    account_two.issue_card("herman card").unwrap();
    log!("Deposited 20.0 into Account Two");
    log!("Issued \"Herman Card\" for Account Two");

    let id_one = bank.open_account(account_one).unwrap();
    let id_two = bank.open_account(account_two).unwrap();
    log!("Opened \"Account One\"");
    log!("Opened \"Account Two\"");

    match bank.transfer(id_one, id_two, 250.0) {
        Ok(_) => log!("Transfer successful"),
        Err(error) => error!("Transfer failed: {}", error),
    }

    error!("error error error test");

    if let Some(herman_account) = bank.accounts.inner.get_mut(&id_two) {
        match herman_account.person.update_name("hermanerku") {
            Ok(_) => log!("Name updated successfully"),
            Err(error) => error!("Name update failed: {}", error),
        }
    }

    match bank.update_phone_number(id_two, 48885005) {
        Ok(_) => log!("Phone number updated successfully"),
        Err(error) => error!("Phone number update failed: {}", error),
    }

    // log!(
    //     "Bank Status: {}",
    //     serde_json::to_string_pretty(&bank.accounts()).unwrap()
    // );

    if let Some(account) = bank.accounts.find_by_id(id_one) {
        log!(
            "Found account: {}",
            serde_json::to_string_pretty(account).unwrap()
        );
    }
}
