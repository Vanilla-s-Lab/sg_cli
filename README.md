## Steam Guard CLI

Access your Steam Guard token locally.

### Usage

- Put a config file to your $HOME, this supported by https://github.com/dirs-dev/dirs-rs#example.
- Named the config file `.sg_cli.toml`, example is
  here: https://github.com/Vanilla-s-Lab/sg_cli/blob/master/example/sg_cli.toml.
- Run sg_cli, it will shows current Steam Guard & expire time, this supported by https://github.com/SFort/steam_guard.

### Extra hint

- you also can use `sg_cli | clipboard` to copy the verification code : )
- The CLI tools `clipboard-cli`: https://github.com/sindresorhus/clipboard-cli, that was useful.

### Credit

https://github.com/steevp/UpdogFarmer/blob/master/app/src/main/java/com/steevsapps/idledaddy/steam/SteamGuard.java  
Rust lib & cli app: https://github.com/SFort/steam_guard & https://github.com/simonsmh/RSteamGuard.
