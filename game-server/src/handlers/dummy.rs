use paste::paste;
use sr_proto::cmd::*;

macro_rules! dummy_map {
    ($($cmd:ident);* $(;)?) => {
        paste! {
            pub fn check_dummy(cmd: u16) -> Option<u16> {
                Some(match cmd {
                    $(
                        [<$cmd:upper _CS_REQ>] => [<$cmd:upper _SC_RSP>],
                    )*
                    _ => return None,
                })
            }
        }
    };
}

dummy_map![
    get_level_reward_taken_list;
    get_rogue_score_reward_info;
    query_product_info;
    get_quest_data;
    get_quest_record;
    get_cur_assist;
    get_rogue_handbook_data;
    get_daily_active_info;
    get_fight_activity_data;
    get_multiple_drop_info;
    get_player_return_multi_drop_info;
    get_share_data;
    get_treasure_dungeon_activity_data;
    player_return_info_query;
    get_player_board_data;
    get_activity_schedule_config;
    get_mission_data;
    get_challenge;
    get_cur_challenge;
    get_rogue_info;
    get_expedition_data;
    get_jukebox_data;
    sync_client_res_version;
    daily_first_meet_pam;
    get_museum_info;
    get_login_activity;
    get_raid_info;
    get_trial_activity_data;
    get_boxing_club_info;
    get_npc_status;
    text_join_query;
    get_secret_key_info;
    get_video_version_key;
    get_cur_battle_info;
    get_phone_data;
    interact_prop;
    finish_talk_mission;
    get_recharge_gift_info;
];
