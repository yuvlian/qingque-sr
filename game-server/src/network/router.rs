use crate::handlers::*;
use crate::network::packet;
use paste::paste;
use sr_proto::cmd::*;

macro_rules! handle {
    ($($handler:ident);* $(;)?) => {
        paste! {
            pub fn ping_pong(cmd: u16, body: &[u8]) -> Vec<u8> {
                match cmd {
                    $(
                        [<$handler:upper _CS_REQ>] => packet::encode_packet(
                            [<$handler:upper _SC_RSP>],
                            $handler::handle(body)
                        ),
                    )*
                    _ => Vec::with_capacity(0),
                }
            }
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
