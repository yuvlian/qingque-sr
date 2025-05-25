// use crate::packet::PacketSink;
// use crate::util::chat::{FRIEND_CHAT_HISTORY, FRIEND_UID, PLAYER_UID};
// use crate::util::cur_timestamp_ms;
// use sr_proto::cmd::{
//     AVATAR_PATH_CHANGED_NOTIFY, GET_PRIVATE_CHAT_HISTORY_SC_RSP, PLAYER_SYNC_SC_NOTIFY,
//     REVC_MSG_SC_NOTIFY, SEND_MSG_SC_RSP, SYNC_LINEUP_NOTIFY,
// };
// use sr_proto::prost::Message;
// use sr_proto::{
//     Avatar, AvatarPathChangedNotify, AvatarSync, AvatarType, ChatMessageData,
//     GetPrivateChatHistoryCsReq, GetPrivateChatHistoryScRsp, LineupAvatar, LineupInfo, MsgType,
//     MultiPathAvatarInfo, MultiPathAvatarType, PlayerSyncScNotify, RevcMsgScNotify, SendMsgCsReq,
//     SendMsgScRsp, SpBarInfo, SyncLineupNotify,
// };

// pub async fn get_private_chat_history(req: &[u8], sink: &mut PacketSink) {
//     let req = GetPrivateChatHistoryCsReq::decode(req).unwrap_or_default();
//     let rsp = GetPrivateChatHistoryScRsp {
//         contact_side: req.contact_side,
//         target_side: FRIEND_UID,
//         chat_message_list: FRIEND_CHAT_HISTORY
//             .iter()
//             .map(|h| ChatMessageData {
//                 message_type: MsgType::CustomText.into(),
//                 create_time: cur_timestamp_ms(),
//                 content: h.to_string(),
//                 sender_id: FRIEND_UID,
//                 ..Default::default()
//             })
//             .collect(),
//         ..Default::default()
//     };

//     sink.push_message(GET_PRIVATE_CHAT_HISTORY_SC_RSP, rsp);
// }

// pub async fn send_msg(req: &[u8], sink: &mut PacketSink) {
//     let req = SendMsgCsReq::decode(req).unwrap_or_default();
//     let command: Vec<String> = req
//         .message_text
//         .trim()
//         .split_whitespace()
//         .map(|l| l.to_lowercase())
//         .collect();

//     if command.len() < 2 {
//         let notify = RevcMsgScNotify {
//             message_type: req.message_type,
//             message_text: String::from("Invalid command"),
//             extra_id: req.extra_id,
//             target_uid: PLAYER_UID,
//             source_uid: FRIEND_UID,
//             chat_type: req.chat_type,
//             hnbepabnbng: req.hnbepabnbng,
//             ..Default::default()
//         };

//         sink.push_message(REVC_MSG_SC_NOTIFY, notify);

//         return;
//     }

//     let mut valid_command = false;

//     match command[0].as_ref() {
//         "mc" if command.len() >= 3 => {
//             let new_mc = match (command[1].as_ref(), command[2].as_ref()) {
//                 ("man", "physical") => MultiPathAvatarType::BoyWarriorType,
//                 ("man", "fire") => MultiPathAvatarType::BoyKnightType,
//                 ("man", "imaginary") => MultiPathAvatarType::BoyShamanType,
//                 ("man", "ice") => MultiPathAvatarType::BoyMemoryType,
//                 ("woman", "physical") => MultiPathAvatarType::GirlWarriorType,
//                 ("woman", "fire") => MultiPathAvatarType::GirlKnightType,
//                 ("woman", "imaginary") => MultiPathAvatarType::GirlShamanType,
//                 ("woman", "ice") => MultiPathAvatarType::GirlMemoryType,
//                 _ => MultiPathAvatarType::None,
//             };

//             if !(new_mc == MultiPathAvatarType::None) {
//                 let notify = AvatarPathChangedNotify {
//                     base_avatar_id: 8001,
//                     cur_multi_path_avatar_type: new_mc.into(),
//                 };

//                 sink.push_message(AVATAR_PATH_CHANGED_NOTIFY, notify);

//                 let notify = PlayerSyncScNotify {
//                     multi_path_avatar_info_list: vec![MultiPathAvatarInfo {
//                         avatar_id: new_mc.into(),
//                         rank: 6,
//                         ..Default::default()
//                     }],
//                     avatar_sync: Some(AvatarSync {
//                         avatar_list: vec![Avatar {
//                             promotion: 6,
//                             rank: 6,
//                             exp: 0,
//                             level: 80,
//                             base_avatar_id: 8001,
//                             ..Default::default()
//                         }],
//                     }),
//                     ..Default::default()
//                 };

//                 sink.push_message(PLAYER_SYNC_SC_NOTIFY, notify);

//                 valid_command = true;
//             }
//         }
//         "march" => {
//             let new_march = match command[1].as_ref() {
//                 "ice" => MultiPathAvatarType::Mar7thKnightType,
//                 "imaginary" => MultiPathAvatarType::Mar7thRogueType,
//                 _ => MultiPathAvatarType::None,
//             };

//             if !(new_march == MultiPathAvatarType::None) {
//                 let notify = AvatarPathChangedNotify {
//                     base_avatar_id: 1001,
//                     cur_multi_path_avatar_type: new_march.into(),
//                 };

//                 sink.push_message(AVATAR_PATH_CHANGED_NOTIFY, notify);

//                 let notify = PlayerSyncScNotify {
//                     multi_path_avatar_info_list: vec![MultiPathAvatarInfo {
//                         avatar_id: new_march.into(),
//                         rank: 6,
//                         ..Default::default()
//                     }],
//                     avatar_sync: Some(AvatarSync {
//                         avatar_list: vec![Avatar {
//                             promotion: 6,
//                             rank: 6,
//                             exp: 0,
//                             level: 80,
//                             base_avatar_id: 1001,
//                             ..Default::default()
//                         }],
//                     }),
//                     ..Default::default()
//                 };

//                 sink.push_message(PLAYER_SYNC_SC_NOTIFY, notify);

//                 valid_command = true;
//             }
//         }
//         "lineup" => {
//             let new_lineup_ids: Vec<u32> = serde_json::from_str(&command[1]).unwrap_or_default();

//             if new_lineup_ids.len() != 0 {
//                 let av_list: Vec<LineupAvatar> = new_lineup_ids
//                     .iter()
//                     .enumerate()
//                     .map(|(i, id)| LineupAvatar {
//                         id: *id,
//                         hp: 10000,
//                         slot: i as u32,
//                         satiety: 0,
//                         sp_bar: Some(SpBarInfo {
//                             cur_sp: 0,
//                             max_sp: 10000,
//                         }),
//                         avatar_type: AvatarType::AvatarFormalType.into(),
//                     })
//                     .collect();

//                 let lineup_info = LineupInfo {
//                     name: String::from("yulianteam"),
//                     avatar_list: av_list,
//                     plane_id: 20101,
//                     ..Default::default()
//                 };

//                 let notify = SyncLineupNotify {
//                     lineup: Some(lineup_info),
//                     ..Default::default()
//                 };

//                 sink.push_message(SYNC_LINEUP_NOTIFY, notify);
//                 valid_command = true;
//             }
//         }
//         _ => {}
//     }

//     let notify = RevcMsgScNotify {
//         message_type: req.message_type,
//         message_text: req.message_text,
//         extra_id: req.extra_id,
//         target_uid: FRIEND_UID,
//         source_uid: PLAYER_UID,
//         chat_type: req.chat_type,
//         hnbepabnbng: req.hnbepabnbng,
//         ..Default::default()
//     };

//     sink.push_message(REVC_MSG_SC_NOTIFY, notify);

//     let notify = RevcMsgScNotify {
//         message_type: req.message_type,
//         message_text: if valid_command {
//             String::from("OK")
//         } else {
//             String::from("Invalid command")
//         },
//         extra_id: req.extra_id,
//         target_uid: PLAYER_UID,
//         source_uid: FRIEND_UID,
//         chat_type: req.chat_type,
//         hnbepabnbng: req.hnbepabnbng,
//         ..Default::default()
//     };

//     sink.push_message(REVC_MSG_SC_NOTIFY, notify);

//     sink.push_message(SEND_MSG_SC_RSP, SendMsgScRsp::default());
// }
