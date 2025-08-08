

pub mod accounts {
    use std::io::{self, Write};
    use serde::{Serialize,Deserialize};
    use arboard::Clipboard;
    use crate::file_handler::file_handler::{save_entries_json};

    #[derive(Debug,Serialize,Deserialize)]
    pub struct AccountDetails {
        email: String,
        username: String,
        password: String,
    }

    // Related to making Accounts serializable
    fn init_clipboard() -> Clipboard {
        Clipboard::new().expect("Failed to initialize clipboard")
    }

    #[derive(Serialize, Deserialize)]
    pub struct Accounts {
        pub entries: Vec<AccountDetails>,
        #[serde(skip, default = "init_clipboard")]
        clipboard: Clipboard,
    }
    
    impl Accounts {
        pub fn new() -> Self {
            Self {
                entries: Vec::new(),
                clipboard: Clipboard::new().unwrap(),
            }
        }

        
        pub fn list_accounts(&self) {
            for (i,entry) in self.entries.iter().enumerate() {
                println!("{i}: Email->{email}\nusername->{username}\nPassword->********\n",i=i,email=entry.email,username=entry.username);
            }
        }

        pub fn add_account(&mut self) {
            let mut input = String::new();
            
            print!("Enter username: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let username= input.trim().to_owned();
            input.clear();
            

            print!("Enter email: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let email= input.trim().to_owned();
            input.clear();

            print!("Enter password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let password= input.trim().to_owned();
            input.clear();

            let new_account = AccountDetails {
                username,
                email,
                password
            };

            self.entries.push(new_account);
            let _ = save_entries_json(&self.entries);

        }

        pub fn copy_to_clipboard(&mut self) {

            let mut input = String::new();
            print!("Enter an id from the accounts list: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let selected_id: usize = input.trim().parse().unwrap();
            input.clear();

            if selected_id > self.entries.len()
            {
                println!("id not in account entries!");
            }else {
                let account_pass = self.entries[selected_id].password.to_owned();
                self.clipboard.set_text(account_pass).unwrap();
                println!("Password copied to clipboard!");
            }
        }
    }

}

