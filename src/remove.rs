use clap::Args;

/// Remove all accounts or specific account
#[derive(Args)]
pub struct Remove {
    pub account: Option<String>
}

pub fn remove_all_accounts() {
    println!("Removing all accounts...");
    println!("All accounts removed");
}

pub fn remove_account(account: &String) {
    println!("Removing account {account}");
    println!("{account} removed")
}