use crate::cmd::{
    avatar::*, battle::*, chat::*, friend::*, item::*, lineup::*, mission::*, player::*, scene::*,
};
use amia_packet::net_packet::NetPacket;
use paste::paste;
use sr_proto::prost::Message;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use tokio::net::TcpStream;

pub struct PacketSink(Vec<Box<[u8]>>);

impl PacketSink {
    pub fn with_capacity(n: usize) -> Self {
        Self(Vec::with_capacity(n))
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline(always)]
    pub fn push_message<T: Message>(&mut self, cmd: u16, message: T) {
        let message = message.encode_to_vec();
        self.push_raw(cmd, message);
    }

    #[inline(always)]
    pub fn push_raw(&mut self, cmd: u16, body: Vec<u8>) {
        let packet = NetPacket {
            cmd,
            body,
            head: Vec::new(),
        };
        self.0.push(Box::<[u8]>::from(packet))
    }

    #[inline(always)]
    pub async fn write_all(&self, stream: &mut TcpStream) -> Result<()> {
        for p in &self.0 {
            stream.write_all(p).await?;
        }
        Ok(())
    }
}

macro_rules! dummy_map {
    ($($cmd:ident);* $(;)?) => {
        paste! {
            #[inline(always)]
            pub fn check_dummy(cmd: u16) -> Option<u16> {
                Some(match cmd {
                    $(
                        sr_proto::cmd::[<$cmd:upper _CS_REQ>] => sr_proto::cmd::[<$cmd:upper _SC_RSP>],
                    )*
                    _ => return None,
                })
            }
        }
    };
}

macro_rules! handle_map {
    ($($handler:ident);* $(;)?) => {
        paste! {
            #[inline(always)]
            pub async fn handle_packet(req_cmd: u16, req_body: &[u8], packet_sink: &mut PacketSink) {
                match req_cmd {
                    $(
                        sr_proto::cmd::[<$handler:upper _CS_REQ>] => $handler(req_body, packet_sink).await,
                    )*
                    c => {
                        tracing::warn!("Unhandled cmd: {}", c);
                    }
                }
            }
        }
    }
}

handle_map![
    // avatar
    get_avatar_data;
    set_avatar_enhanced_id;
    // battle
    pve_battle_result;
    start_cocoon_stage;
    // chat
    get_private_chat_history;
    send_msg;
    // friend
    get_friend_list_info;
    get_friend_login_info;
    // item
    get_bag;
    // lineup
    change_lineup_leader;
    get_all_lineup_data;
    get_cur_lineup_data;
    // mission
    get_mission_status;
    // player
    get_basic_info;
    player_get_token;
    player_heart_beat;
    player_login;
    player_login_finish;
    // scene
    get_cur_scene_info;
    get_scene_map_info;
    scene_entity_move;
];

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
