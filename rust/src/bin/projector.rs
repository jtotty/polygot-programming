use clap::Parser;
use rust::{config::Config, options::Options};

use anyhow::Result;

fn main() -> Result<()> {
    let opts: Config = Options::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(());
}
