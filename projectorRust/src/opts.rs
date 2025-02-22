use std::path::PathBuf;

use clap::Parser;



#[derive(Parser)]
#[clap()]
#[derive(Debug)]
pub struct Opts {
    pub args: Vec<String>,
    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,
    #[clap(short = 'p', long = "pwd")]
    pub pwd : Option<PathBuf>
}