use crate::handlers::*;
use crate::network::packet;
use paste::paste;
use sr_proto::cmd::*;

macro_rules! enc {
    ($body:expr, $rsp_cmd:expr, $handler:path) => {
        packet::encode_packet($rsp_cmd, $handler($body).await)
    };
}

macro_rules! router {
    ($($req:ident $rsp:ident $handler:path);* $(;)?) => {
        pub async fn ping_pong(cmd: u16, body: &[u8]) -> Vec<u8> {
            match cmd {
                $(
                    $req => enc!(body, $rsp, $handler),
                )*
                _ => Vec::with_capacity(0),
            }
        }
    };
}

macro_rules! handle {
    ($($handler:ident);* $(;)?) => {
        paste! {
            router![
                $(
                    [<$handler:upper _CS_REQ>] [<$handler:upper _SC_RSP>] $handler::handle;
                )*
            ];
        }
    };
}

handle![
    player_get_token;
    player_login;
    player_login_finish;
    player_heart_beat;
    get_basic_info;
    get_avatar_data;
    get_multi_path_avatar_info;
    get_bag;
    get_mission_status;
    get_cur_lineup_data;
    get_all_lineup_data;
    get_cur_scene_info;
    get_scene_map_info;
    pve_battle_result;
    start_cocoon_stage;
];
