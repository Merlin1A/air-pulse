use std::env;

pub fn init_environment() {
    dotenv::dotenv().expect("Failed to read .env file");
}

