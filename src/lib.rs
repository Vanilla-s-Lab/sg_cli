use clap::Parser;
use serde::Deserialize;

pub const CONFIG_FILE: &str = ".sg_cli.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sg_cli: SgCli,
}

#[derive(Deserialize, Debug)]
pub struct SgCli {
    pub secret: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Specific secret from Steam Guard
    #[clap(short, long)]
    pub secret: Option<String>,
}
