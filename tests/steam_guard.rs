use std::fs;

use tap::Pipe;

use sg_cli::Config;

#[test]
fn fake_steam_guard() {
    let fake_secret: Config = fs::read_to_string("./example/sg_cli.toml")
        .unwrap().pipe(|it| { toml::from_str(it.as_str()) }).unwrap();

    let sg_result =
        steam_guard::from_secret(fake_secret.sg_cli.secret.as_str());

    assert!(sg_result.is_err());
}
