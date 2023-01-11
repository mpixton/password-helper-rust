# Overview
This CLI is a tool to help anyone remember their passwords. It provides a simple interface for a user to
add, edit, remove, view all accounts that they have stored. It also gives a command, `check` to test the 
user's knowledge of their previously stored passwords. All passwords and accounts are only stored locally, 
passwords are hashed according to an industry standard hashing algorithm (Argon2), and no account or 
password is ever sent over the internet. 

# Recommended Usage
Add any accounts (used very generically here) that you would like to remember the password to with the 
`add` command. These might be online accounts, GPG/SSH keys, or user logins. Periodically test your 
knowledge with the `check` command. If an account is no longer relevant, or you feel you have the 
password memorized, great! Remove it with the `remove` command. Use `list` to see a list of all accounts 
you have stored. And finally, if a password changes, use the `edit` command. If at any time you need help, 
type `-h` for help with the whole utility, `--help` for more verbose output, and `help <command>` for help 
with that specific command. 