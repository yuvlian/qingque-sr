use crate::app::{
    request::ConfigSdkComboReq,
    response::{ConfigSdkComboRsp, ConfigSdkComboVals, IRsp},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_query): Query<ConfigSdkComboReq>) -> Json<IRsp<ConfigSdkComboRsp>> {
    Json(IRsp::<ConfigSdkComboRsp> {
        data: Some(ConfigSdkComboRsp {
            vals: ConfigSdkComboVals {
                pay_payco_centered_host: "bill.payco.com".to_string(),
                list_price_tierv2_enable: "false".to_string(),
                enable_register_autologin: "true".to_string(),
                account_list_page_enable: "true".to_string(),
                network_report_config: r#"{ "enable": 1, "status_codes": [206], "url_paths": ["dataUpload", "red_dot"] }"#.to_string(),
                enable_logout_ann_redpoint: "true".to_string(),
                telemetry_config: r#"{ "dataupload_enable": 1 }"#.to_string(),
                kibana_pc_config: r#"{ "enable": 1, "level": "Info", "modules": ["download"] }"#.to_string(),
                new_forgotpwd_page_enable: "true".to_string(),
                webview_rendermethod_config: r#"{"useLegacy":true}"#.to_string(),
                new_register_page_enable: "true".to_string(),
                enable_user_center_v2: "false".to_string(),
                h5log_filter_config: r#"{ "function": { "event_name": [ "info_get_cps", "notice_close_notice", "info_get_uapc", "report_set_info", "info_get_channel_id", "info_get_sub_channel_id" ] } }"#.to_string(),
                enable_web_dpi: "true".to_string(),
            }
        }),
        ..Default::default()
    })
}
