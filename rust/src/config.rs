use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use crate::options::Options;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Options> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Options) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd,
        });
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut value = value;

        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect args to exist");

        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "add operation add expects 2 arguements but got {}",
                    value.len() - 1
                ));
            }

            let mut args = value.drain(1..=2);
            return Ok(Operation::Add(
                args.next().expect("to exist"),
                args.next().expect("to exist"),
            ));
        }

        if term == "remove" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "remove operation add expects 1 arguements but got {}",
                    value.len() - 1
                ));
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!(
                "print operation expects 0 or 1 arguements but got {}",
                value.len()
            ));
        }

        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(value) = config {
        return Ok(value);
    }

    let location = std::env::var("HOME").context("Unable to get HOME directory")?;
    let mut location = PathBuf::from(location);

    location.push(".projector.json");

    return Ok(location);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("could not get current_dir")?);
}
