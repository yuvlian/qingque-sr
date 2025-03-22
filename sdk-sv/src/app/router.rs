use crate::ArcState;
use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{auth, dispatch, misc};

pub fn auth_router() -> Router<ArcState> {
    let mut r = Router::new();

    r = r.route("/my_register", get(auth::my_register::get));

    r = r.route("/my_register", post(auth::my_register::post));

    r = r.route(
        "/{product}/combo/granter/login/beforeVerify",
        post(auth::combo_before_verify::post),
    );

    r = r.route(
        "/account/risky/api/check",
        post(auth::risky_api_check::post),
    );

    r = r.route(
        "/{product}/mdk/shield/api/login",
        post(auth::shield_api_login::post),
    );

    r = r.route(
        "/{product}/mdk/shield/api/verify",
        post(auth::shield_api_verify::post),
    );

    r = r.route(
        "/{product}/combo/granter/login/v2/login",
        post(auth::combo_granter_login::post),
    );

    r
}

pub fn dispatch_router() -> Router<ArcState> {
    let mut r = Router::new();

    r = r.route("/query_dispatch", get(dispatch::query_dispatch::get));

    r = r.route("/query_gateway", post(dispatch::query_gateway::get));

    r
}

pub fn misc_router() -> Router<ArcState> {
    let mut r = Router::new();

    r = r.route("/admin/mi18n/{*remainder}", get(misc::admin_mi18n::get));

    r = r.route(
        "/{product}/combo/granter/api/compareProtocolVersion",
        post(misc::compare_protocol_version::post),
    );

    r = r.route(
        "/data_abtest_api/config/experiment/list",
        post(misc::config_experiment_list::post),
    );

    r = r.route(
        "/combo/box/api/config/sdk/combo",
        get(misc::config_sdk_combo::get),
    );

    r = r.route(
        "/device-fp/api/getExtList",
        get(misc::device_get_ext_list::get),
    );

    r = r.route("/device-fp/api/getFp", post(misc::device_get_fp::post));

    r = r.route(
        "/{product}/mdk/agreement/api/getAgreementInfos",
        get(misc::get_agreement_infos::get),
    );

    r = r.route(
        "/common/{product}/announcement/api/getAlertAnn",
        get(misc::get_alert_ann::get),
    );

    r = r.route(
        "/common/{product}/announcement/api/getAlertPic",
        get(misc::get_alert_pic::get),
    );

    r = r.route(
        "/common/{product}/announcement/api/getAnnList",
        get(misc::get_ann_list::get),
    );

    r = r.route(
        "/{product}/combo/granter/api/getConfig",
        get(misc::granter_api_get_config::get),
    );

    r = r.route("/common/h5log/log/batch", post(misc::h5log::post));

    r = r.route(
        "/{product}/combo/red_dot/list",
        post(misc::red_dot_list::post),
    );

    r = r.route("/sdk/dataUpload", post(misc::sdk_data_upload::post));

    r = r.route(
        "/{product}/mdk/shield/api/loadConfig",
        get(misc::shield_api_load_config::get),
    );

    r = r.route(
        "/combo/box/api/config/sw/precache",
        get(misc::sw_precache::get),
    );

    r
}
