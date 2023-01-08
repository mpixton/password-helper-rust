use clap::Args;

/// Check your knowledge of all accounts or a specific account's password
#[derive(Args)]
pub struct Check {
    pub account: Option<String>
}

pub fn check_all_accounts() {
    println!("Checking accounts...");
    println!("Checking account: BW");
    println!("Checking account: Linux-Work");
}

pub fn check_account(account: &String) {
    println!("Checking account {account}");
    println!("{account} checked");
}