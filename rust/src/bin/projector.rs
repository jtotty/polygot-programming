use clap::Parser;
use rust::{
    config::{Config, Operation},
    options::Options,
    projector::Projector,
};

use anyhow::Result;

fn main() -> Result<()> {
    let config: Config = Options::parse().try_into()?;
    let mut proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = proj.get_all_values();
            let value = serde_json::to_string(&value)?;
            println!("{}", value);
        }

        Operation::Print(Some(k)) => {
            proj.get_value(&k).map(|x| {
                println!("{}", x);
            });
        }

        Operation::Add(k, v) => {
            proj.set_value(k, v);
            let _ = proj.save();
        }

        Operation::Remove(k) => {
            proj.remove_value(&k);
            let _ = proj.save();
        }
    }

    return Ok(());
}
