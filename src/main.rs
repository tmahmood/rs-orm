extern crate postgres;
extern crate rand;
extern crate dotenv;
extern crate chrono;

use dotenv::dotenv;
use std::env;

#[macro_use]
mod lib;
mod app;

#[cfg( test )]
mod testing;

fn main() {
    println!("Hello db");
}

