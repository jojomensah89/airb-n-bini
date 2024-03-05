use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String = set_database();
}

pub fn set_database() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}
