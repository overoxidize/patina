//! # Patina
//!
//! Patina is a Restful Api server in Sync Rust for aggregating information on
//! both cryptocurrencies and cryptocurrency exchanges, similar to CoinMarketCap.
//! 
#![feature(trivial_bounds)]
#![warn(missing_docs)]
#[macro_use]
extern crate diesel;
mod logger;
mod exchange;
mod cryptocurrency;
mod db;
mod schema;
type Err = Box<dyn std::error::Error>;

fn main() -> Result<(), Err> {

    dotenv::dotenv()?;
    println!("Hello, world!");

    logger::log_config_init()?;

    Ok(())
}
