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
