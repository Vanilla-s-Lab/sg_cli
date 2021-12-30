use std::fs;

use tap::Pipe;

use sg_cli::Config;

#[test]
fn fake_steam_guard() {
    let fake_config = read_config();
    let sg_result =
        steam_guard::from_secret(&fake_config.sg_cli.secret);

    assert!(sg_result.is_err());
}

fn read_config() -> Config {
    fs::read_to_string("./example/sg_cli.toml").unwrap()
        .pipe(|it| { toml::from_str(&it) }).unwrap()
}
