use std::{
    path::{ PathBuf},
};

use anyhow::{anyhow, Context, Result};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> std::result::Result<Self, Self::Error> {
        let operation = value.args.try_into()?;
        let config = get_Config(value.config)?;
        let pwd = get_Pwd(value.pwd)?;

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

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "operation add expects 2 arguments but got {}",
                    value.len() - 1
                ));
            }

            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exists"),
                drain.next().expect("to exists"),
            ));
        }

        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "operation add expects 1 arguments but got {}",
                    value.len() - 1
                ));
            }
            let drain = value.pop().expect("to exists");
            return Ok(Operation::Remove(drain));
        }

        if value.len() > 1 {
            return Err(anyhow!(
                "operation print expect 0 or 1 argument but recieved {}",
                value.len() - 1
            ));
        }

        let drain = value.pop().expect("to exists");
        return Ok(Operation::Print(Some(drain)));
    }
}

fn get_Config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    let loc = std::env::var("XDG_CONFIG_HOME").context("unable to get XDG_CONFIG_HOME")?;
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_Pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    let pwd = std::env::current_dir().context("Error getting current directory")?;
    return Ok(pwd);
}
