use std::fs;

use tap::{Pipe, Tap};

use sg_cli::{Config, CONFIG_FILE};

fn main() {
    let config: Config = dirs::home_dir().expect("Error while getting home_dir. ")
        .tap_mut(|ct| { ct.push(CONFIG_FILE) }).pipe(fs::read_to_string)
        .expect(format!("Failed to read the config file {}. ", CONFIG_FILE).as_str())
        .pipe(|cc| { toml::from_str(cc.as_str()) }).expect("Failed to parse the config file. ");

    let steam_secret = config.sg_cli.secret;

    let sg_code = steam_guard::from_secret(&steam_secret)
        .unwrap_or_else(|err| { panic!("{}", err) });
    let expire_sec = steam_guard::expires_in_sec();

    println!("Steam Guard Code: {}, expire in {} s. ", sg_code, expire_sec);
}
