use axum::extract::Query;
use cfg_utility::hotfix::GameVersion;
use cfg_utility::server::ServerConfig;
use sr_proto::Gateserver;
use sr_proto::Message;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Gateway {
    pub version: Option<String>,
}

pub async fn handle(Query(q): Query<Gateway>) -> String {
    let server_config = ServerConfig::from_file("_cfg/server.toml");
    let game_version = GameVersion::from_file("_cfg/hotfix.json");
    let hotfix = game_version.get_hotfix_by_version(&q.version);

    rbase64::encode(
        &Gateserver {
            // hotfix
            lua_url: hotfix.lua_url,
            lua_version: hotfix.lua_version,
            ex_resource_url: hotfix.ex_resource_url,
            asset_bundle_url: hotfix.asset_bundle_url,

            // we're not using kcp.
            use_tcp: true,
            ip: server_config.game_server_host,
            port: server_config.game_server_port,

            // let's just bruteforce the bool fields.
            unk1: true,
            unk2: true,
            unk3: true,
            unk4: true,
            unk5: true,
            unk6: true,
            unk7: true,
            // use_tcp was unk8
            // unk8: true,
            unk9: true,
            unk10: true,
            unk11: true,
            unk12: true,
            unk13: true,
            unk14: true,
            unk15: true,
            ..Default::default()
        }
        .encode_to_vec(),
    )
}
