use crate::ArcState;
use crate::handlers::{auth, dispatch, misc};
use axum::{
    Router,
    routing::{get, post},
};
use paste::paste;

macro_rules! gen_router {
    ($prefix:ident; $($route:literal $method:ident $handler:ident);* $(;)?) => {
        paste! {
            pub fn [<$prefix _router>]() -> Router<ArcState> {
                let mut r = Router::new();

                $(
                    r = r.route($route, $method($prefix::$handler::$method));
                )*

                r
            }
        }
    };
}

gen_router![
    auth;
    "/account/register" get account_register;
    "/accountSystemAuditFE/{*remainder}" get account_register;
    "/login-platform/{*remainder}" get account_register;
    "/account/register" post account_register;
    "/account/risky/api/check" post risky_api_check;
    "/{product}/combo/granter/login/beforeVerify" post combo_before_verify;
    "/{product}/mdk/shield/api/login" post shield_api_login;
    "/{product}/mdk/shield/api/verify" post shield_api_verify;
    "/{product}/combo/granter/login/v2/login" post combo_granter_login
];

gen_router![
    dispatch;
    "/query_dispatch" get query_dispatch;
    "/query_gateway" get query_gateway
];

gen_router![
    misc;
    "/admin/mi18n/{*remainder}" get admin_mi18n;
    "/common/{product}/announcement/api/getAlertAnn" get get_alert_ann;
    "/common/{product}/announcement/api/getAlertPic" get get_alert_pic;
    "/common/{product}/announcement/api/getAnnList" get get_ann_list;
    "/common/h5log/log/batch" post h5log;
    "/combo/box/api/config/sdk/combo" get config_sdk_combo;
    "/combo/box/api/config/sw/precache" get sw_precache;
    "/data_abtest_api/config/experiment/list" post config_experiment_list;
    "/device-fp/api/getExtList" get device_get_ext_list;
    "/device-fp/api/getFp" post device_get_fp;
    "/{product}/combo/granter/api/compareProtocolVersion" post compare_protocol_version;
    "/{product}/combo/granter/api/getConfig" get granter_api_get_config;
    "/{product}/mdk/agreement/api/getAgreementInfos" get get_agreement_infos;
    "/{product}/mdk/shield/api/loadConfig" get shield_api_load_config;
    "/{product}/combo/red_dot/list" post red_dot_list;
    "/sdk/dataUpload" post sdk_data_upload
];
