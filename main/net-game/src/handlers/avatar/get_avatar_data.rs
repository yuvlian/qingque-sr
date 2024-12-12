use net_msg::pb::{Avatar, GetAvatarDataScRsp};
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    GetAvatarDataScRsp {
        is_get_all: true,
        avatar_list: vec![Avatar {
            promotion: 6,
            rank: 6,
            exp: 0,
            level: 80,
            base_avatar_id: 1201,
            ..Default::default()
        }],
        retcode: 0,
        unlocked_skin_id_list: Vec::with_capacity(0),
    }
    .encode_to_vec()
}
