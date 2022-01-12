use std::{fs, io};
use std::io::Write;

use atty::Stream;
use clap::Parser;
use tap::{Pipe, Tap};

use sg_cli::{Args, Config, CONFIG_FILE};

fn main() {
    let args: Args = Args::parse();

    let steam_secret = match args.secret {
        None => read_cfg().sg_cli.secret,
        Some(secret) => secret,
    };

    let sg_code = steam_guard::from_secret(&steam_secret).unwrap();
    let expire_sec = steam_guard::expires_in_sec();

    sg_code_string(&sg_code, expire_sec).as_bytes()
        .pipe(|it| { io::stdout().write_all(it) })
        .unwrap(); // #[warn(unused_must_use)]
}

fn sg_code_string(sg_code: &str, expire_sec: u64) -> String {
    match atty::is(Stream::Stdout) {
        true => format!("Steam Guard Code: \"{}\", expire in {} s. ", sg_code, expire_sec),
        false => format!("{}", sg_code), // For piping, like `sg_cli | clipboard`.
    }
}

fn read_cfg() -> Config {
    dirs::home_dir().expect("Error while getting home_dir. ")
        .tap_mut(|it| { it.push(CONFIG_FILE) }).pipe(fs::read_to_string)
        .expect(&format!("Failed to read the config file {}. ", CONFIG_FILE))
        .pipe(|it| { toml::from_str(&it) }).expect("Failed to parse the config file. ")
}
