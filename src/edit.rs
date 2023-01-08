use clap::Args;

/// Edit an account's password
#[derive(Args)]
pub struct Edit {
    pub account: String,
} 

pub fn edit_account(account: &String) {
    println!("Editing account {account}");
}