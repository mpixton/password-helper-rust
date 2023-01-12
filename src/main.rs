// !!! Argon2 does not enable std feature flag by default.

//! Simple CLI for testing a user on their passwords.
//!
//! List of all Subcommands:
//! - List - list all accounts
//! - Edit <account> - edit given account password
//! - Add <account> - add an account
//! - Check - test the user on all accounts in the database
//! - Check [account] - test the user on a specific account
//! - Remove - remove all accounts from the database
//! - Remove [account] - remove a specific account from the database

use clap::{command, Arg, ArgAction, Args, Command, Parser, Subcommand, FromArgMatches};
use log::LevelFilter;
use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use sqlx::SqlitePool;

mod add;
mod check;
mod database;
mod edit;
mod error;
mod list;
mod password;
mod remove;

/// Utility to help memorize passwords
///
/// Program is a CLI for adding, editing, removing, listing, and most importantly, testing a
/// user's knowledge of their stored passwords. All passwords are hashed using Argon2 before
/// being stored in a local-only `SQLite` database. Even if your computer is compromised, all
/// your passwords are safe behind industry-grade hashing and the passwords do no leave your
/// device.
#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

/// All possible Subcommands to interact with the program.
#[derive(Subcommand)]
enum Subcommands {
    Add(add::Command),
    Check(check::Command),
    Edit(edit::Command),
    List(list::Command),
    Remove(remove::Command),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Command::new("password helper").arg(
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::Count),
    );

    // Parse the CLI struct
    let cli = Cli::augment_args(cli);

    let matches = cli.get_matches();

    let log_level = match matches.get_count("verbose") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let stdout = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("sqlx", LevelFilter::Off))
        .build(Root::builder().appender("stdout").build(log_level))
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    // Setup the db, if needed, and return the db connection options
    let db_url = database::setup_db().await?;

    let pool = SqlitePool::connect(db_url).await?;

    let subcommand = Subcommands::from_arg_matches(&matches).map_err(|err| err.exit()).unwrap(); 

    match subcommand {
        Subcommands::List(_) => list::list_all_accounts(&pool).await,
        Subcommands::Add(data) => add::add_account(&pool, &data.account).await,
        Subcommands::Edit(data) => edit::edit_account(&pool, &data.account).await,
        Subcommands::Check(data) => match &data.account {
            Some(account) => check::check_account(&pool, account).await,
            None => check::check_all_accounts(&pool).await,
        },
        Subcommands::Remove(data) => match &data.account {
            Some(account) => remove::remove_account(&pool, account).await,
            None => remove::remove_all_accounts(&pool).await,
        },
    }?;

    Ok(())
}
