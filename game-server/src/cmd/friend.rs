// use crate::packet::PacketSink;
// use crate::util::chat::{FRIEND_CHAT_BUBBLE_ID, FRIEND_ICON_ID, FRIEND_UID};
// use sr_proto::cmd::{GET_FRIEND_LIST_INFO_SC_RSP, GET_FRIEND_LOGIN_INFO_SC_RSP};
// use sr_proto::{
//     FriendOnlineStatus, FriendSimpleInfo, GetFriendListInfoScRsp, GetFriendLoginInfoScRsp,
//     PlatformType, PlayerSimpleInfo,
// };

// pub async fn get_friend_list_info(_req: &[u8], sink: &mut PacketSink) {
//     let rsp = GetFriendListInfoScRsp {
//         friend_list: vec![FriendSimpleInfo {
//             is_marked: true,
//             remark_name: String::from("QingqueSR"),
//             player_info: Some(PlayerSimpleInfo {
//                 uid: FRIEND_UID,
//                 platform: PlatformType::Pc.into(),
//                 online_status: FriendOnlineStatus::Online.into(),
//                 head_icon: FRIEND_ICON_ID,
//                 chat_bubble_id: FRIEND_CHAT_BUBBLE_ID,
//                 level: 70,
//                 nickname: String::from("Server"),
//                 signature: String::from("https://github.com/yuvlian/qingque-sr"),
//                 ..Default::default()
//             }),
//             ..Default::default()
//         }],
//         ..Default::default()
//     };

//     sink.push_message(GET_FRIEND_LIST_INFO_SC_RSP, rsp);
// }

// pub async fn get_friend_login_info(_req: &[u8], sink: &mut PacketSink) {
//     let rsp = GetFriendLoginInfoScRsp {
//         // friend_uid_list
//         iihdbinopmg: vec![FRIEND_UID],
//         ..Default::default()
//     };
//     sink.push_message(GET_FRIEND_LOGIN_INFO_SC_RSP, rsp);
// }
