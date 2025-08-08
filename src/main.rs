mod manager;
mod file_handler;

use std::io::{self, Write};
use manager::accounts;

use file_handler::file_handler::load_json;
 
const MAX_CHOICE: i8 = 5;

// Called to load in json passwords on start-up
fn accounts_init(accounts: &mut accounts::Accounts) {
    let _ = load_json(accounts);
}

fn menu_banner() -> i8 {
    println!("
    Please select an option from the menu:\n
    1.) List accounts
    2.) Add a new account
    3.) Retrieve password
    4.) About
    5.) Exit
    ");

    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut input);

    match input.trim().parse::<i8>() {
        Ok(num) => num,
        Err(_) => -1,
    }
}

fn main() {
    
    println!("Welcome to RustPass!");
    let mut choice;
    let mut account_holder = accounts::Accounts::new();
    accounts_init(&mut account_holder);

    loop {
        loop {
            choice = menu_banner();
            if choice >= 0 && choice <= MAX_CHOICE {
                break
            }
            println!("Invalid choice selected!");
        }

        println!("You chose: {}", choice);

        match choice {
            1 => account_holder.list_accounts(),
            2 => account_holder.add_account(),
            3 => account_holder.copy_to_clipboard(),
            5 => break,
            _ => println!("Invalid choice, getting here is impossible!"),
        }
        
    }
}
