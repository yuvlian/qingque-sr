use net_msg::pb::{AmountInfo, AvatarType, GetCurLineupDataScRsp, LineupAvatar, LineupInfo};
use net_msg::Trait;

pub async fn handle(_req: &[u8]) -> Vec<u8> {
    GetCurLineupDataScRsp {
        retcode: 0,
        lineup: Some(LineupInfo {
            name: String::from("smolteam"),
            avatar_list: vec![LineupAvatar {
                id: 1201,
                hp: 10000,
                slot_type: 0,
                satiety: 0,
                sp: Some(AmountInfo {
                    cur_amount: 0,
                    max_amount: 10000,
                }),
                avatar_type: AvatarType::AvatarFormalType.into(),
            }],
            plane_id: 20101,
            ..Default::default()
        }),
    }
    .encode_to_vec()
}
