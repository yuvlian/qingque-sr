# QingqueSR 

Ever wanted a private server that is quick to update when the game updates, but doesn't sacrifice too much features?

A server that's modular enough and feature complete-ish for battle simulation?

You're in the right place!

## Tutorial

This tutorial assumes you have basic knowledge of a terminal and traffic redirection with a proxy.

Also, if config files are missing, the server will fallback to default config. This means you can delete all files in `_config` folder.

You only have to worry about `config.json` because that's the config file for battle. It's obtainable from https://srtools.pages.dev/

### From Prebuilt (Windows only)

1. Download the prebuilt that matches your SR version in https://github.com/f2pqingque/sr/releases
2. Extract the zip
5. Edit config files in `_config` if necessary (check the README in that folder)
4. Run `net-game.exe` and `net-sdk.exe`

### From Source

1. Install Rust
2. Clone this repository
3. `cd` into `main`
4. Put your proto & cmdid file in `net-msg`
5. Edit config files in `_config` if necessary (check the README in that folder)
6. `cargo run --release --bin net-game`
7. `cargo run --release --bin net-sdk`

## Credits
- Yulian: QingqueSR Developer
- amizing25: SrTools Author
