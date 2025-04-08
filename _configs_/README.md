## `avatar.toml`
configuration for lineup, march/trailblazer path, and owned avatar. 

the lineup here only affects overworld lineup and not battle.

changing anything here will only take effect after restarting the game.

NOTE: your avatar multipath here must match with the multipath you're using for battle, otherwise the game will break.

## `config.json`
configuration for battle, obtainable from https://srtools.pages.dev/

you can use `srtools-manager` to manage config.json easily

if you're building from source do: `cargo run --release srtools-manager`

## `hotfix.json`
configuration for hotfix links.

hotfix is not needed if you play on production, as you can obtain the update by logging into the official server.

if you need a tool to grab this, i have one: https://github.com/yuvlian/fetch-hotfix-hsr (no prebuilt)

## `scene.toml`
configuration for player position, calyx position, and map.

## `server.toml`
configuration for where the servers should bind to.
