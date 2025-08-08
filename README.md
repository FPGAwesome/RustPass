# RustPass

RustPass is a simple command-line password manager written in Rust.  
It was built as a refresher going through "The Book" as well as trying out a few different crates.
While the copy-to-clipboard function is kinda nice, I don't think it'll be replacing my regular password manager
anytime soon. Still, fun to write up.

## Features
- Store account entries with username, email, and password
- List stored accounts (passwords are masked)
- Copy a password to the clipboard
- Save and load accounts from a JSON file (very, very secure right?)

## Requirements
- Rust (latest stable)
- Cargo

## Installation
Clone the repository and build the project:
```bash
git clone https://github.com/FPGAwesome/RustPass.git
cd RustPass
cargo build
```
