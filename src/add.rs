use clap::Args;

/// Add an account
#[derive(Args)]
pub struct Add {
    pub account: String
}

pub fn add_account(account: &String){
    println!("Add account {account}");
}