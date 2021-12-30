use std::{fs, io};
use std::io::Write;

use atty::Stream;
use tap::{Pipe, Tap};

use sg_cli::{Config, CONFIG_FILE};

fn main() {
    let config: Config = dirs::home_dir().expect("Error while getting home_dir. ")
        .tap_mut(|it| { it.push(CONFIG_FILE) }).pipe(fs::read_to_string)
        .expect(&format!("Failed to read the config file {}. ", CONFIG_FILE))
        .pipe(|it| { toml::from_str(&it) }).expect("Failed to parse the config file. ");

    let steam_secret = config.sg_cli.secret;

    let sg_code = steam_guard::from_secret(&steam_secret).unwrap();
    let expire_sec = steam_guard::expires_in_sec();

    sg_code_string(&sg_code, expire_sec).as_bytes()
        .pipe(|it| { io::stdout().write_all(it) });
}

fn sg_code_string(sg_code: &str, expire_sec: u64) -> String {
    match atty::is(Stream::Stdout) {
        true => format!("Steam Guard Code: \"{}\", expire in {} s. ", sg_code, expire_sec),
        false => format!("{}", sg_code), // For piping, like `sg_cli | clipboard`.
    }
}
