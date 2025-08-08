pub mod file_handler {
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    
    use crate::manager::accounts::{Accounts, AccountDetails};

    pub fn load_json(account_ref: &mut Accounts) -> Result<bool, Box<dyn std::error::Error>>{
        let f = File::open("accounts.json")?;
        let reader = BufReader::new(f);

        account_ref.entries = serde_json::from_reader(reader)?;

        Ok(true)
    }

    pub fn save_entries_json(entries: &Vec<AccountDetails>) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::create("accounts.json")?;
        let writer = BufWriter::new(f);
        serde_json::to_writer_pretty(writer, entries)?;
        Ok(())
    }
}