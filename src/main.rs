use std::fs;

use tap::Tap;

const CONFIG_FILE: &str = ".sg_cli.toml";

fn main() {
    let ss_path = dirs::home_dir()
        .expect("Error while getting home_dir. ")
        .tap_mut(|hdb| { hdb.push(CONFIG_FILE) });

    let error_msg = format!("Error while reading file {:?}. ", ss_path);
    let steam_secret = fs::read_to_string(ss_path)
        .expect(error_msg.as_str()).trim().to_string();

    let sg_code = steam_guard::from_secret(&steam_secret)
        .unwrap_or_else(|err| { panic!("{}", err) });
    let expire_sec = steam_guard::expires_in_sec();

    println!("Steam Guard Code: {}, expire in {} s. ", sg_code, expire_sec);
}
