extern crate postgres;
extern crate rand;
extern crate dotenv;
extern crate chrono;

use dotenv::dotenv;
use std::env;

mod app;
mod lib;

#[cfg( test )]
mod testing;

fn main() {
    println!("Hello db");
}

