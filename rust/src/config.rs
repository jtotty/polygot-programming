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

#[derive(Debug, PartialEq)]
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

    // Linux
    if let Ok(home) = std::env::var("XDG_CONFIG_HOME") {
        let mut home = PathBuf::from(home);
        home.push("projector");
        home.push("projector.json");
        return Ok(home);
    }

    // Windows
    if let Ok(home) = std::env::var("USERPROFILE") {
        let mut home = PathBuf::from(home);
        home.push("projector");
        home.push("projector.json");
        return Ok(home);
    }

    return Err(anyhow!("Unable to find config home location"));
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    return Ok(std::env::current_dir().context("could not get current_dir")?);
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{config::Operation, options::Options};

    use super::Config;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Options {
            args: vec![],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));
        return Ok(());
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Options {
            args: vec![String::from("foo")],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some(String::from("foo"))));
        return Ok(());
    }

    #[test]
    fn test_add_key_value() -> Result<()> {
        let opts: Config = Options {
            args: vec![
                String::from("add"),
                String::from("foo"),
                String::from("bar"),
            ],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(
            opts.operation,
            Operation::Add(String::from("foo"), String::from("bar"))
        );
        return Ok(());
    }

    #[test]
    fn test_remove_key() -> Result<()> {
        let opts: Config = Options {
            args: vec![String::from("remove"), String::from("foo")],
            pwd: None,
            config: None,
        }
        .try_into()?;

        assert_eq!(opts.operation, Operation::Remove(String::from("foo")));
        return Ok(());
    }
}
