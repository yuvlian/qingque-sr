use crate::app::{
    request::ShieldApiLoadConfigReq,
    response::{IRsp, ShieldApiLoadConfigRsp, ThirdpartyLoginConfigs, TokenInfo},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(
    Query(query): Query<ShieldApiLoadConfigReq>,
) -> Json<IRsp<ShieldApiLoadConfigRsp>> {
    Json(IRsp::<ShieldApiLoadConfigRsp> {
        data: Some(ShieldApiLoadConfigRsp {
            id: 24,
            game_key: query.game_key,
            client: query.client,
            identity: "I_IDENTITY".to_string(),
            scene: "S_NORMAL".to_string(),
            name: "崩坏RPG".to_string(),
            disable_regist: true,
            thirdparty: vec![
                "fb".to_string(),
                "tw".to_string(),
                "gl".to_string(),
                "ap".to_string(),
            ],
            thirdparty_login_configs: ThirdpartyLoginConfigs {
                fb: TokenInfo {
                    token_type: "TK_GAME_TOKEN".to_string(),
                    game_token_expires_in: 2592000,
                },
                gl: TokenInfo {
                    token_type: "TK_GAME_TOKEN".to_string(),
                    game_token_expires_in: 604800,
                },
                tw: TokenInfo {
                    token_type: "TK_GAME_TOKEN".to_string(),
                    game_token_expires_in: 2592000,
                },
                ap: TokenInfo {
                    token_type: "TK_GAME_TOKEN".to_string(),
                    game_token_expires_in: 604800,
                },
            },
            logo_height: "0".to_string(),
            logo_width: "0".to_string(),
            hoyoplay_auth_login: true,
            ..Default::default()
        }),
        ..Default::default()
    })
}
