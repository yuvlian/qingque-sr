use crate::handlers::*;
use crate::network::packet;
use net_msg::cmd::*;

macro_rules! enc {
    ($body:expr, $rsp_cmd:expr, $handler:path) => {
        packet::encode_packet($rsp_cmd, $handler($body).await)
    };
}

pub async fn ping_pong(cmd: u16, body: &[u8]) -> Vec<u8> {
    match cmd {
        PLAYER_GET_TOKEN_CS_REQ => {
            enc!(body, PLAYER_GET_TOKEN_SC_RSP, player_get_token::handle)
        }
        PLAYER_LOGIN_CS_REQ => {
            enc!(body, PLAYER_LOGIN_SC_RSP, player_login::handle)
        }
        PLAYER_LOGIN_FINISH_CS_REQ => {
            enc!(
                body,
                PLAYER_LOGIN_FINISH_SC_RSP,
                player_login_finish::handle
            )
        }
        PLAYER_HEART_BEAT_CS_REQ => {
            enc!(body, PLAYER_HEART_BEAT_SC_RSP, player_heart_beat::handle)
        }
        GET_BASIC_INFO_CS_REQ => {
            enc!(body, GET_BASIC_INFO_SC_RSP, get_basic_info::handle)
        }
        GET_AVATAR_DATA_CS_REQ => {
            enc!(body, GET_AVATAR_DATA_SC_RSP, get_avatar_data::handle)
        }
        GET_MULTI_PATH_AVATAR_INFO_CS_REQ => {
            enc!(
                body,
                GET_MULTI_PATH_AVATAR_INFO_SC_RSP,
                get_multi_path_avatar_info::handle
            )
        }
        GET_BAG_CS_REQ => {
            enc!(body, GET_BAG_SC_RSP, get_bag::handle)
        }
        GET_MISSION_STATUS_CS_REQ => {
            enc!(body, GET_MISSION_STATUS_SC_RSP, get_mission_status::handle)
        }
        GET_CUR_LINEUP_DATA_CS_REQ => {
            enc!(
                body,
                GET_CUR_LINEUP_DATA_SC_RSP,
                get_cur_lineup_data::handle
            )
        }
        GET_ALL_LINEUP_DATA_CS_REQ => {
            enc!(
                body,
                GET_ALL_LINEUP_DATA_SC_RSP,
                get_all_lineup_data::handle
            )
        }
        GET_CUR_SCENE_INFO_CS_REQ => {
            enc!(body, GET_CUR_SCENE_INFO_SC_RSP, get_cur_scene_info::handle)
        }
        P_V_E_BATTLE_RESULT_CS_REQ => {
            enc!(
                body,
                P_V_E_BATTLE_RESULT_SC_RSP,
                p_v_e_battle_result::handle
            )
        }
        START_COCOON_STAGE_CS_REQ => {
            enc!(body, START_COCOON_STAGE_SC_RSP, start_cocoon_stage::handle)
        }
        _ => Vec::with_capacity(0),
    }
}
