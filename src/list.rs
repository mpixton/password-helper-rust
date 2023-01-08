use clap::Args;

/// List all accounts
#[derive(Args)]
pub struct List {}

pub fn list_all_accounts() {
    println!("Listing all accounts...");
    println!("Bw");
}