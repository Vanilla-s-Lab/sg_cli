use std::fs;

const STEAM_SECRET: &str = ".steam_secret";

fn main() {
    let mut home_dir_buf = dirs::home_dir()
        .expect("Error while getting home_dir. ");
    home_dir_buf.push(STEAM_SECRET);

    let error_msg = format!("Error while reading file {:?}. ", home_dir_buf);
    let steam_secret = fs::read_to_string(home_dir_buf)
        .expect(error_msg.as_str()).trim().to_string();

    let sg_code = steam_guard::from_secret(&steam_secret)
        .unwrap_or_else(|err| { panic!("{}", err) });
    let expire_sec = steam_guard::expires_in_sec();

    println!("Steam Guard Code: {}, expire in {} s. ", sg_code, expire_sec);
}
