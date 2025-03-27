#![allow(dead_code)]
use serde::Deserialize;
use serde::Serialize;

// NOTE: some things may be censored with hashtags, I don't want DMCA
// Update: nvm i dont care, too lazy to change it back tho
// extra note: listPriceTier is missing

// ------------------------------------------------------------------------- //

// Helper generic struct for some responses
#[derive(Serialize)]
pub struct IRsp<T: Default + Serialize> {
    pub retcode: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Default + Serialize> Default for IRsp<T> {
    fn default() -> Self {
        Self {
            retcode: 0,
            message: String::from("OK"), // "success", if h5log
            data: None,
        }
    }
}

impl<T: Default + Serialize> IRsp<T> {
    // Helper function for a successful response
    pub fn ok(data: T) -> Self {
        IRsp {
            retcode: 0,
            message: String::from("OK"),
            data: Some(data),
        }
    }

    pub fn internal_error() -> Self {
        IRsp {
            retcode: -1,
            message: String::from("Internal Server Error"),
            data: None,
        }
    }

    pub fn custom_error(retcode: i32, message: String) -> Self {
        IRsp {
            retcode,
            message,
            data: None,
        }
    }
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// URL: https://minor-api-os.#########.com/common/h5log/log/batch?topic=plat_explog_sdk_v2
impl<T: Default + Serialize> IRsp<T> {
    pub fn h5log_rsp() -> Self {
        Self {
            retcode: 0,
            message: String::from("success"),
            data: None,
        }
    }
}

// ------------------------------------------------------------------------- //

// im too lazy to add comments, just check request.rs
pub type QueryDispatchRsp = String;
pub type QueryGatewayRsp = String;

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Note: Should probably just let this pass through proxy.
// URL:
// - https://webstatic.#########.com/admin/mi18n/plat_oversea/m2020030410/m2020030410-version.json
// - https://webstatic.#########.com/admin/mi18n/plat_os/m09291531181441/m09291531181441-version.json
// Example Rsp: {"version": 108}
#[derive(Default, Deserialize, Serialize)]
pub struct AdminMi18nPlatRsp {
    pub version: i32,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL:
// - https://#####-sdk-os.#########.com/#####_global/combo/granter/api/compareProtocolVersion?
// Example Rsp: IRsp.data = {"modified": false, "protocol": null}
#[derive(Default, Serialize)]
pub struct CompareProtocolVersionRsp {
    pub modified: bool,
    pub protocol: Option<serde_json::Value>,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// URL: https://#####-sdk-os-static.#########.com/#####_global/mdk/shield/api/loadConfig?client=3&game_key=#####_global
// Example Rsp: IRsp.data = {"id":24,"game_key":"#####_global","client":"PC","identity":"I_IDENTITY","guest":false,"ignore_versions":"","scene":"S_NORMAL","name":"崩坏RPG","disable_regist":false,"enable_email_captcha":false,"thirdparty":["fb","tw","gl","ap"],"disable_mmt":false,"server_guest":false,"thirdparty_ignore":{},"enable_ps_bind_account":false,"thirdparty_login_configs":{"fb":{"token_type":"TK_GAME_TOKEN","game_token_expires_in":2592000},"gl":{"token_type":"TK_GAME_TOKEN","game_token_expires_in":604800},"tw":{"token_type":"TK_GAME_TOKEN","game_token_expires_in":2592000},"ap":{"token_type":"TK_GAME_TOKEN","game_token_expires_in":604800}},"initialize_firebase":false,"bbs_auth_login":false,"bbs_auth_login_ignore":[],"fetch_instance_id":false,"enable_flash_login":false,"enable_logo_18":false,"logo_height":"0","logo_width":"0","enable_cx_bind_account":false,"firebase_blacklist_devices_switch":false,"firebase_blacklist_devices_version":0,"####lab_auth_login":false,"####lab_auth_login_ignore":[],"####play_auth_login":true}
#[derive(Default, Serialize)]
pub struct ShieldApiLoadConfigRsp {
    pub id: i32,
    pub game_key: String,
    pub client: String,
    pub identity: String,
    pub guest: bool,
    pub ignore_versions: String,
    pub scene: String,
    pub name: String,
    pub disable_regist: bool,
    pub enable_email_captcha: bool,
    pub thirdparty: Vec<String>,
    pub disable_mmt: bool,
    pub server_guest: bool,
    pub thirdparty_ignore: ThirdpartyIgnore,
    pub enable_ps_bind_account: bool,
    pub thirdparty_login_configs: ThirdpartyLoginConfigs,
    pub initialize_firebase: bool,
    pub bbs_auth_login: bool,
    pub bbs_auth_login_ignore: Vec<Option<serde_json::Value>>,
    pub fetch_instance_id: bool,
    pub enable_flash_login: bool,
    pub enable_logo_18: bool,
    pub logo_height: String,
    pub logo_width: String,
    pub enable_cx_bind_account: bool,
    pub firebase_blacklist_devices_switch: bool,
    pub firebase_blacklist_devices_version: i32,
    pub hoyolab_auth_login: bool,
    pub hoyolab_auth_login_ignore: Vec<Option<serde_json::Value>>,
    pub hoyoplay_auth_login: bool,
}

#[derive(Default, Serialize)]
pub struct ThirdpartyIgnore {}

#[derive(Default, Serialize)]
pub struct ThirdpartyLoginConfigs {
    pub fb: TokenInfo,
    pub gl: TokenInfo,
    pub tw: TokenInfo,
    pub ap: TokenInfo,
}

#[derive(Default, Serialize)]
pub struct TokenInfo {
    pub token_type: String,
    pub game_token_expires_in: i64,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// URL: https://sg-public-data-api.#########.com/device-fp/api/getExtList?platform=3
// Example Rsp: IRsp.data = {"code":200,"msg":"ok","ext_list":["cpuName","deviceModel","deviceName","deviceType","deviceUID","gpuID","gpuName","gpuAPI","gpuVendor","gpuVersion","gpuMemory","osVersion","cpuCores","cpuFrequency","gpuVendorID","isGpuMultiTread","memorySize","screenSize","engineName","addressMAC","packageVersion"],"pkg_list":[],"pkg_str":"/vK5WTh5SS3SAj8Zm0qPWg=="}
#[derive(Default, Serialize)]
pub struct DeviceGetExtListRsp {
    pub code: i32,
    pub msg: String,
    pub ext_list: Vec<String>,
    pub pkg_list: Vec<Option<serde_json::Value>>,
    pub pkg_str: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// URL: https://#####-sdk-os-static.#########.com/#####_global/combo/granter/api/getConfig?app_id=11&channel_id=1&client_type=3
// Example Rsp: IRsp.data = {"protocol":true,"qr_enabled":false,"log_level":"INFO","announce_url":"https://sdk.#########.com/#####/announcement/index.html?sdk_presentation_style=fullscreen\u0026game=#####\u0026game_biz=#####_global\u0026sdk_screen_transparent=true\u0026auth_appid=announcement\u0026authkey_ver=1\u0026version=2.34\u0026sign_type=2#/","push_alias_type":0,"disable_ysdk_guard":false,"enable_announce_pic_popup":false,"app_name":"崩坏RPG","qr_enabled_apps":{"bbs":false,"cloud":false},"qr_app_icons":{"app":"","bbs":"","cloud":""},"qr_cloud_display_name":"","enable_user_center":true,"functional_switch_configs":{}}
#[derive(Default, Serialize)]
pub struct GranterApiGetConfigRsp {
    pub protocol: bool,
    pub qr_enabled: bool,
    pub log_level: String,
    pub announce_url: String,
    pub push_alias_type: i32,
    pub disable_ysdk_guard: bool,
    pub enable_announce_pic_popup: bool,
    pub app_name: String,
    pub qr_enabled_apps: QrEnabledApps,
    pub qr_app_icons: QrAppIcons,
    pub qr_cloud_display_name: String,
    pub enable_user_center: bool,
    pub functional_switch_configs: FunctionalSwitchConfigs,
}

#[derive(Default, Serialize)]
pub struct FunctionalSwitchConfigs {}

#[derive(Default, Serialize)]
pub struct QrAppIcons {
    pub app: String,
    pub bbs: String,
    pub cloud: String,
}

#[derive(Default, Serialize)]
pub struct QrEnabledApps {
    pub bbs: bool,
    pub cloud: bool,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// URL: https://sdk-os-static.#########.com/combo/box/api/config/sdk/combo?biz_key=#####_global&client_type=3
// Example Rsp: IRsp.data = {"vals":{"pay_payco_centered_host":"bill.payco.com","list_price_tierv2_enable":"false\n","enable_register_autologin":"true","account_list_page_enable":"true","network_report_config":"{ \"enable\": 1, \"status_codes\": [206], \"url_paths\": [\"dataUpload\", \"red_dot\"] }\n","enable_logout_ann_redpoint":"true","telemetry_config":"{\n \"dataupload_enable\": 1,\n}","kibana_pc_config":"{ \"enable\": 1, \"level\": \"Info\",\"modules\": [\"download\"] }\n","new_forgotpwd_page_enable":"true","webview_rendermethod_config":"{\"useLegacy\":true}","new_register_page_enable":"true","enable_user_center_v2":"false","h5log_filter_config":"{\n\t\"function\": {\n\t\t\"event_name\": [\"info_get_cps\", \"notice_close_notice\", \"info_get_uapc\", \"report_set_info\", \"info_get_channel_id\", \"info_get_sub_channel_id\"]\n\t}\n}","enable_web_dpi":"true"}}
#[derive(Default, Serialize)]
pub struct ConfigSdkComboRsp {
    pub vals: ConfigSdkComboVals,
}

#[derive(Default, Serialize)]
pub struct ConfigSdkComboVals {
    pub pay_payco_centered_host: String,
    pub list_price_tierv2_enable: String,
    pub enable_register_autologin: String,
    pub account_list_page_enable: String,
    pub network_report_config: String,
    pub enable_logout_ann_redpoint: String,
    pub telemetry_config: String,
    pub kibana_pc_config: String,
    pub new_forgotpwd_page_enable: String,
    pub webview_rendermethod_config: String,
    pub new_register_page_enable: String,
    pub enable_user_center_v2: String,
    pub h5log_filter_config: String,
    pub enable_web_dpi: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://sg-public-data-api.#########.com/device-fp/api/getFp
// Example Rsp: IRsp.data = {"device_fp":"x","code":200,"msg":"ok"}
#[derive(Default, Serialize)]
pub struct DeviceGetFpRsp {
    pub device_fp: String,
    pub code: i32,
    pub msg: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://abtest-api-data-sg.#########.com/data_abtest_api/config/experiment/list
// Example Rsp: IRsp.data = [{"code":1000,"type":2,"config_id":"200","period_id":"6592_784","version":"1","configs":{"####pass_enable":"false"},"sceneWhiteList":false,"experimentWhiteList":false},{"code":1000,"type":2,"config_id":"184","period_id":"6554_748","version":"1","configs":{"userCenterType":"v1"},"sceneWhiteList":false,"experimentWhiteList":false}]
pub type ConfigExperimentListRsp = Vec<ConfigExperiment>;

#[derive(Default, Serialize)]
pub struct ConfigExperiment {
    pub code: i32,
    pub r#type: i32,
    pub config_id: String,
    pub period_id: String,
    pub version: String,
    pub configs: UserExperimentConfigs,
    #[serde(rename = "sceneWhiteList")]
    pub scene_white_list: bool,
    #[serde(rename = "experimentWhiteList")]
    pub experiment_white_list: bool,
}

#[derive(Default, Serialize)]
pub struct UserExperimentConfigs {
    pub hoyopass_enable: Option<String>,
    #[serde(rename = "userCenterType")]
    pub user_center_type: Option<String>,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// URL: https://sdk-os-static.#########.com/combo/box/api/config/sw/precache?biz=#####_global&client=3
// Example Rsp: IRsp.data = {"vals":{"url":"https://sdk.#########.com/sw.html","enable":"true"}}
#[derive(Default, Serialize)]
pub struct SwPrecacheRsp {
    pub vals: SwPrecacheVals,
}

#[derive(Default, Serialize)]
pub struct SwPrecacheVals {
    pub url: String,
    pub enable: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json | bytes (i think)
// Extends: IRsp
// URL: https://sg-#####-api.#########.com/common/#####_global/announcement/api/getAnnList?bundle_id=#####_global&channel_id=1&game=#####&game_biz=#####_global&lang=en&platform=pc&region=prod_official_asia
// Example Rsp: IRsp.data = {"list":[{"list":[{"ann_id":186,"title":"######: #### #### Fair Gaming Declaration","subtitle":"######: #### #### Fair Gaming Declaration","banner":"https://sdk.#########.com/upload/ann/2023/04/04/bbab09d84af4bbb294dc9350fdbaf69c_6005675597601035971.png","content":"","type_label":"Notices","tag_label":"重要","tag_icon":"https://sdk.#########.com/upload/announcement/2023/01/05/605ab5f6643d822ec6cdb158a66c81ad_3387186949421905950.png","login_alert":1,"lang":"en-us","start_time":"2023-04-04 00:00:00","end_time":"2035-06-07 06:00:00","type":4,"remind":0,"alert":0,"tag_start_time":"2000-01-02 15:04:05","tag_end_time":"2030-01-02 15:04:05","remind_ver":1,"has_content":true,"extra_remind":0,"tag_icon_hover":"","logout_remind":0,"logout_remind_ver":1}],"type_id":4,"type_label":"Notices"}],"total":1,"type_list":[{"id":4,"name":"公告","mi18n_name":"Notices"}],"alert":false,"alert_id":0,"timezone":8,"t":"1742222317_ae6295e8279662a881eb9beb19144112_d41d8cd98f00b204e9800998ecf8427e","pic_list":[{"type_list":[],"type_id":3,"type_label":"News"}],"pic_total":0,"pic_type_list":[{"id":3,"name":"资讯","mi18n_name":"News"}],"pic_alert":false,"pic_alert_id":0,"static_sign":"","banner":""}
#[derive(Default, Serialize)]
pub struct GetAnnListRsp {
    pub list: Vec<AnnouncementCategory>,
    pub total: i32,
    pub type_list: Vec<AnnouncementType>,
    pub alert: bool,
    pub alert_id: i32,
    pub timezone: i32,
    pub t: String,
    pub pic_list: Vec<PictureCategory>,
    pub pic_total: i32,
    pub pic_type_list: Vec<AnnouncementType>,
    pub pic_alert: bool,
    pub pic_alert_id: i32,
    pub static_sign: String,
    pub banner: String,
}

#[derive(Default, Serialize)]
pub struct AnnouncementCategory {
    pub list: Vec<Announcement>,
    pub type_id: i32,
    pub type_label: String,
}

#[derive(Default, Serialize)]
pub struct Announcement {
    pub ann_id: i32,
    pub title: String,
    pub subtitle: String,
    pub banner: String,
    pub content: String,
    pub type_label: String,
    pub tag_label: String,
    pub tag_icon: String,
    pub login_alert: i32,
    pub lang: String,
    pub start_time: String,
    pub end_time: String,
    pub r#type: i32,
    pub remind: i32,
    pub alert: i32,
    pub tag_start_time: String,
    pub tag_end_time: String,
    pub remind_ver: i32,
    pub has_content: bool,
    pub extra_remind: i32,
    pub tag_icon_hover: String,
    pub logout_remind: i32,
    pub logout_remind_ver: i32,
}

#[derive(Default, Serialize)]
pub struct PictureCategory {
    pub type_list: Vec<Option<serde_json::Value>>,
    pub type_id: i32,
    pub type_label: String,
}

#[derive(Default, Serialize)]
pub struct AnnouncementType {
    pub id: i32,
    pub name: String,
    pub mi18n_name: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://#####-sdk-os.#########.com/#####_global/mdk/shield/api/verify?
// Example Rsp: IRsp.data = {"account":{"uid":"1","name":"y","email":"a@a.a","mobile":"1","is_email_verify":"0","realname":"","identity_card":"","token":"x","safe_mobile":"","facebook_name":"Y","google_name":"","twitter_name":"","game_center_name":"","apple_name":"","sony_name":"","tap_name":"","country":"ID","reactivate_ticket":"","area_code":"**","device_grant_ticket":"","steam_name":"","unmasked_email":"","unmasked_email_type":0,"cx_name":""},"device_grant_required":false,"safe_moblie_required":false,"realperson_required":false,"realname_operation":"None"}
#[derive(Default, Serialize)]
pub struct ShieldApiVerifyRsp {
    pub account: ShieldAccount,
    pub device_grant_required: bool,
    pub safe_moblie_required: bool,
    pub realperson_required: bool,
    pub realname_operation: String,
}

#[derive(Default, Serialize)]
pub struct ShieldAccount {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub is_email_verify: String,
    pub realname: String,
    pub identity_card: String,
    pub token: String,
    pub safe_mobile: String,
    pub facebook_name: String,
    pub google_name: String,
    pub twitter_name: String,
    pub game_center_name: String,
    pub apple_name: String,
    pub sony_name: String,
    pub tap_name: String,
    pub country: String,
    pub reactivate_ticket: String,
    pub area_code: String,
    pub device_grant_ticket: String,
    pub steam_name: String,
    pub unmasked_email: String,
    pub unmasked_email_type: i32,
    pub cx_name: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://#####-sdk-os.#########.com/#####_global/combo/granter/login/v2/login?
// Example Rsp: IRsp.data = {"combo_id":"0","open_id":"1","combo_token":"x","data":"{\"guest\":false}","heartbeat":false,"account_type":1,"fatigue_remind":null}
#[derive(Default, Serialize)]
pub struct ComboGranterLoginRsp {
    pub combo_id: String,
    pub open_id: String,
    pub combo_token: String,
    pub data: String,
    pub heartbeat: bool,
    pub account_type: i32,
    pub fatigue_remind: Option<String>,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// URL: https://#####-sdk-os.#########.com/#####_global/mdk/agreement/api/getAgreementInfos?
// Example Rsp: IRsp.data = {"marketing_agreements":[]}
#[derive(Default, Serialize)]
pub struct GetAgreementInfosRsp {
    pub marketing_agreements: Vec<Option<serde_json::Value>>,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// URL:
// - https://sdk-log-upload-os.#########.com/sdk/dataUpload
// - https://#####-log-upload-os.#########.com/sdk/dataUpload
// Example Rsp: {"code": 0}
#[derive(Serialize)]
pub struct SdkDataUploadRsp {
    pub code: i32,
    pub message: Option<String>,
}

impl Default for SdkDataUploadRsp {
    fn default() -> Self {
        Self {
            code: 0,
            message: Some(String::from("OK")),
        }
    }
}

// impl SdkDataUploadRsp {
//     // sdk-log-upload-os.#########.com
//     pub fn with_message() -> Self {
//         Self {
//             code: 0,
//             message: Some(String::from("OK")),
//         }
//     }

//     // #####-log-upload-os.#########.com
//     pub fn without_message() -> Self {
//         Self {
//             code: 0,
//             message: None,
//         }
//     }
// }

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://#####-sdk-os.#########.com/#####_global/combo/granter/login/beforeVerify?
// Example Rsp: IRsp.data = {"is_heartbeat_required":false,"is_realname_required":false,"is_guardian_required":false}
#[derive(Default, Serialize)]
pub struct ComboBeforeVerifyRsp {
    pub is_heartbeat_required: bool,
    pub is_realname_required: bool,
    pub is_guardian_required: bool,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://sg-hkrpg-api.hoyoverse.com/common/hkrpg_global/announcement/api/getAlertAnn?bundle_id=hkrpg_global&channel_id=1&game=hkrpg&game_biz=hkrpg_global&lang=en&level=70&platform=pc&region=prod_official_asia&uid=1
// Example Rsp: IRsp.data = {"alert":false,"alert_id":0,"remind":true,"extra_remind":true}
#[derive(Default, Serialize)]
pub struct GetAlertAnnRsp {
    pub alert: bool,
    pub alert_id: i32,
    pub remind: bool,
    pub extra_remind: bool,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://hkrpg-sdk-os.hoyoverse.com/hkrpg_global/combo/red_dot/list
// Example Rsp: IRsp.data = {"infos":[]}
#[derive(Default, Serialize)]
pub struct RedDotListRsp {
    pub infos: Vec<Option<serde_json::Value>>,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: json
// Extends: IRsp
// Note: this is probably the same as GetAnnListRsp.
// URL: https://sg-hkrpg-api.hoyoverse.com/common/hkrpg_global/announcement/api/getAlertPic?bundle_id=hkrpg_global&channel_id=1&game=hkrpg&game_biz=hkrpg_global&lang=en&level=70&platform=pc&region=prod_official_asia&uid=1
// Example Rsp: IRsp.data = {"total":0,"list":[]}
#[derive(Default, Serialize)]
pub struct GetAlertPicRsp {
    pub total: i32,
    pub list: Vec<Option<serde_json::Value>>,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Response Type: Html
// Note: retcode for http rsp is stupid tbh, status code headers exist
// Note edit: nvm, htmx doesnt like error codes
// URL: http://localhost:.../my_register?username=...&password=...
pub type AccountRegisterRsp = String;

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Extends: IRsp
// URL: https://api-account-os.hoyoverse.com/account/risky/api/check?
// Example Rsp: IRsp.data = {"id":"9e54a9727a014ba4afd2cb2bb4347fe3","action":"ACTION_NONE","geetest":null}
#[derive(Default, Serialize)]
pub struct RiskyApiCheckRsp {
    pub id: String,
    pub action: String,
    pub geetest: Option<serde_json::Value>,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Response Type: json
// Note: Nearly the same shit with shield api verify
// Extends: IRsp
// URL: https://hkrpg-sdk-os.hoyoverse.com/hkrpg_global/mdk/shield/api/login
// Example Rsp: IRsp.data = {"account":{"uid":"1","name":"y","email":"a@a.a","mobile":"1","is_email_verify":"0","realname":"","identity_card":"","token":"x","safe_mobile":"","facebook_name":"Y","google_name":"","twitter_name":"","game_center_name":"","apple_name":"","sony_name":"","tap_name":"","country":"ID","reactivate_ticket":"","area_code":"**","device_grant_ticket":"","steam_name":"","unmasked_email":"","unmasked_email_type":0,"cx_name":""},"device_grant_required":false,"safe_moblie_required":false,"realperson_required":false,"reactivate_required":false,"realname_operation":"None"}
#[derive(Default, Serialize)]
pub struct ShieldApiLoginRsp {
    pub account: ShieldAccount,
    pub device_grant_required: bool,
    pub safe_moblie_required: bool,
    pub realperson_required: bool,
    pub reactivate_required: bool,
    pub realname_operation: String,
}

// ------------------------------------------------------------------------- //
