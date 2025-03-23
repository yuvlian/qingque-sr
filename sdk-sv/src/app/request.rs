#![allow(dead_code)]
use serde::Deserialize;

// NOTE: some things may be censored with hashtags, I don't want DMCA
// Update: nvm i dont care, too lazy to change it back tho
// extra note: listPriceTier is missing

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body, Query
// Note: `data` field might be AES encrypted then base64 encoded like 3rd game dispatch RSPs
// URL: https://minor-api-os.#########.com/common/h5log/log/batch?topic=plat_explog_sdk_v2
#[derive(Deserialize, Clone)]
pub struct H5LogBodyReq {
    pub data: String,
}

#[derive(Deserialize, Clone)]
pub struct H5LogQueryReq {
    pub topic: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// Note: Response is text from protobuf encoded then base64 encode
// URL: globaldp-prod-os01.#########.com /query_dispatch?version=OSPRODWin3.1.0&t=1&language_type=3&platform_type=3&channel_id=1&sub_channel_id=1&is_new_format=1 HTTP/1.1
#[derive(Deserialize, Clone)]
pub struct QueryDispatchReq {
    pub version: String,
    pub t: i64, // time
    pub language_type: i32,
    pub platform_type: i32,
    pub channel_id: i32,
    pub sub_channel_id: i32,
    pub is_new_format: i32,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// Note: Response is text from protobuf encoded then base64 encode
// URL: query_gateway?version=OSPRODWin3.1.0&t=1&uid=1&language_type=3&platform_type=3&dispatch_seed=e910cd6ec2&channel_id=1&sub_channel_id=1&is_need_url=1&game_version=3.1.0&account_type=1&account_uid=1
#[derive(Deserialize, Clone)]
pub struct QueryGatewayReq {
    pub version: String,
    pub t: i64,
    pub uid: u32,
    pub language_type: i32,
    pub platform_type: i32,
    pub dispatch_seed: String,
    pub channel_id: i32,
    pub sub_channel_id: i32,
    pub is_need_url: i32,
    pub game_version: String,
    pub account_type: i32,
    pub account_uid: u32,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://#####-sdk-os.#########.com/#####_global/combo/granter/api/compareProtocolVersion?
#[derive(Deserialize, Clone)]
pub struct CompareProtocolVersionReq {
    pub app_id: String,
    pub channel_id: String,
    pub language: String,
    pub major: String,
    pub minimum: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://#####-sdk-os-static.#########.com/#####_global/mdk/shield/api/loadConfig?client=3&game_key=#####_global
#[derive(Deserialize, Clone)]
pub struct ShieldApiLoadConfigReq {
    pub client: String,
    pub game_key: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://sg-public-data-api.#########.com/device-fp/api/getExtList?platform=3
#[derive(Deserialize, Clone)]
pub struct DeviceGetExtListReq {
    pub platform: i32,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://#####-sdk-os-static.#########.com/#####_global/combo/granter/api/getConfig?app_id=11&channel_id=1&client_type=3
#[derive(Deserialize, Clone)]
pub struct GranterApiGetConfigReq {
    pub app_id: String,
    pub channel_id: i32,
    pub client_type: i32,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://sdk-os-static.#########.com/combo/box/api/config/sdk/combo?biz_key=#####_global&client_type=3
#[derive(Deserialize, Clone)]
pub struct ConfigSdkComboReq {
    pub biz_key: String,
    pub client_type: i32,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://sg-public-data-api.#########.com/device-fp/api/getFp
#[derive(Deserialize, Clone)]
pub struct DeviceGetFpReq {
    pub device_id: String,
    pub seed_id: String,
    pub seed_time: String,
    pub platform: String,
    pub device_fp: String,
    pub app_name: String,
    pub ext_fields: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://abtest-api-data-sg.#########.com/data_abtest_api/config/experiment/list
#[derive(Deserialize, Clone)]
pub struct ConfigExperimentListReq {
    pub app_id: String,
    pub app_sign: String,
    pub uid: String,
    pub scene_id: String,
    pub experiment_id: String,
    pub params: Vec<StringKV>,
}

#[derive(Deserialize, Clone)]
pub struct StringKV {
    pub k: String,
    pub v: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://sdk-os-static.#########.com/combo/box/api/config/sw/precache?biz=#####_global&client=3
#[derive(Deserialize, Clone)]
pub struct SwPrecacheReq {
    pub biz: String,
    pub client: i32,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://sg-#####-api.#########.com/common/#####_global/announcement/api/getAnnList?bundle_id=#####_global&channel_id=1&game=#####&game_biz=#####_global&lang=en&platform=pc&region=prod_official_asia
#[derive(Deserialize, Clone)]
pub struct GetAnnListReq {
    pub bundle_id: String,
    pub channel_id: i32,
    pub game: String,
    pub game_biz: String,
    pub lang: String,
    pub platform: String,
    pub region: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://#####-sdk-os.#########.com/#####_global/mdk/shield/api/verify?
#[derive(Deserialize, Clone)]
pub struct ShieldApiVerifyReq {
    pub uid: String,
    pub token: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://#####-sdk-os.#########.com/#####_global/combo/granter/login/v2/login?
#[derive(Deserialize, Clone)]
pub struct ComboGranterLoginReq {
    pub data: String,
    pub app_id: i32,
    pub channel_id: i32,
    pub device: String,
    pub sign: String,
}

impl ComboGranterLoginReq {
    pub fn parse_data(&self) -> Result<ComboGranterLoginReqData, serde_json::Error> {
        serde_json::from_str(&self.data)
    }
}

#[derive(Deserialize, Clone)]
pub struct ComboGranterLoginReqData {
    pub token: String,
    pub username: String,
    pub uid: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://#####-sdk-os.#########.com/#####_global/mdk/agreement/api/getAgreementInfos?biz_key=#####_global&country_code=ID&token=xxxxxxxxxxxxxxx&uid=1
#[derive(Deserialize, Clone)]
pub struct GetAgreementInfosReq {
    pub biz_key: String,
    pub country_code: String,
    pub token: String,
    pub uid: u32,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://sdk-log-upload-os.#########.com/sdk/dataUpload
pub type SdkDataUploadReq = Vec<SdkUploadData>;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SdkUploadData {
    pub application_id: i32,
    pub application_name: String,
    pub event_id: i32,
    pub event_name: String,
    pub event_time: String,
    pub msg_id: String,
    pub upload_content: UploadContent,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadContent {
    pub device_info: DeviceInfo,
    pub event_time_ms: String,
    pub launch_trace_id: String,
    pub log_info: Option<LogInfo>,
    pub telemetry_info: TelemetryInfo,
    pub user_info: Option<UserInfo>,
    pub version_info: Option<VersionInfo>,
    #[serde(rename = "device_info")]
    pub upload_content_device_info: Option<DeviceInfoAlternate>,
    #[serde(rename = "log_info")]
    pub upload_content_log_info: Option<LogInfoAlternate>,
    #[serde(rename = "user_info")]
    pub upload_content_user_info: Option<UserInfoAlternate>,
    #[serde(rename = "version_info")]
    pub upload_content_version_info: Option<VersionInfoAlternate>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub address_mac: Option<String>,
    pub bundle_id: Option<String>,
    pub cps: Option<String>,
    pub device_id: String,
    pub device_model: String,
    pub device_name: String,
    pub device_sci_x: i32,
    pub device_sci_y: i32,
    #[serde(rename = "device_fp")]
    pub device_fp: Option<String>,
    #[serde(rename = "device_sciX")]
    pub device_info_device_sci_x: Option<i32>,
    #[serde(rename = "device_sciY")]
    pub device_info_device_sci_y: Option<i32>,
    pub dpi: i32,
    pub gpu_mem_size: Option<i64>,
    pub gpu_name: Option<String>,
    pub ip: Option<String>,
    pub isp: Option<String>,
    pub network_type: String,
    #[serde(rename = "network_type")]
    pub device_info_network_type: Option<String>,
    pub platform: i32,
    pub platform_name: String,
    pub processor_count: i32,
    pub processor_frequency: Option<f64>,
    pub processor_type: String,
    pub ram_capacity: i64,
    pub ram_remain: f64,
    #[serde(rename = "registerCPS")]
    pub register_cps: Option<String>,
    pub rom_capacity: Option<i64>,
    pub rom_remain: Option<f64>,
    #[serde(rename = "soft_sciX")]
    pub soft_sci_x: Option<i32>,
    #[serde(rename = "soft_sciY")]
    pub soft_sci_y: Option<i32>,
    pub system_info: String,
    pub system_lang: String,
    pub uapc: Option<String>,
    pub wmac: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogInfo {
    pub action_id: i32,
    pub action_name: String,
    pub c_body: String,
    pub log_time: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryInfo {
    pub area: String,
    pub lifecycle_id: String,
    pub telemetry_version: String,
}

#[derive(Deserialize, Clone)]
pub struct DeviceInfoAlternate {
    pub bundle_id: String,
    pub channel_id: String,
    pub channel_subid: String,
    pub current_cps: String,
    pub device_fp: String,
    pub device_id: String,
    pub device_model: String,
    pub device_name: String,
    pub ip: String,
    pub network_type: String,
    pub os: String,
    pub plat: i32,
    pub register_cps: String,
    pub uapc: String,
}

#[derive(Deserialize, Clone)]
pub struct LogInfoAlternate {
    pub action_id: i32,
    pub action_name: String,
    pub c_body: String,
    pub log_time: String,
    pub platform_type: String,
    pub region: String,
}

#[derive(Deserialize, Clone)]
pub struct UserInfoAlternate {
    pub account_type: String,
    pub level: String,
    pub open_id: String,
    pub tag: String,
    pub uid: String,
}

#[derive(Deserialize, Clone)]
pub struct VersionInfoAlternate {
    pub client_version: String,
    pub log_version: String,
    pub sdk_env: i32,
    pub sdk_version: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub account_id: String,
    pub account_type: String,
    pub channel_id: String,
    pub user_id: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub client_version: String,
    pub log_version: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://#####-sdk-os.#########.com/#####_global/combo/granter/login/beforeVerify?
#[derive(Deserialize, Clone)]
pub struct ComboBeforeVerifyReq {
    pub app_id: String,
    pub channel_id: String,
    pub open_id: String,
    pub combo_token: String,
    pub time: i64,
    pub role: BeforeVerifyRole,
}

#[derive(Deserialize, Clone)]
pub struct BeforeVerifyRole {
    pub region: String,
    pub uid: String,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// URL: https://sg-hkrpg-api.hoyoverse.com/common/hkrpg_global/announcement/api/getAlertAnn?bundle_id=hkrpg_global&channel_id=1&game=hkrpg&game_biz=hkrpg_global&lang=en&level=70&platform=pc&region=prod_official_asia&uid=1
#[derive(Deserialize, Clone)]
pub struct GetAlertAnnReq {
    pub bundle_id: String,
    pub channel_id: i32,
    pub game: String,
    pub game_biz: String,
    pub lang: String,
    pub level: u32,
    pub platform: String,
    pub region: String,
    pub uid: u32,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://hkrpg-sdk-os.hoyoverse.com/hkrpg_global/combo/red_dot/list
#[derive(Deserialize, Clone)]
pub struct RedDotListReq {
    pub uid: String,
    pub region: String,
    pub game_biz: String,
    pub player_level: u32,
}

// ------------------------------------------------------------------------- //

// Request Type: GET
// Parse Target: Query
// Note: this is the exactly the same as GetAlertAnnReq.
// URL: https://sg-hkrpg-api.hoyoverse.com/common/hkrpg_global/announcement/api/getAlertPic?bundle_id=hkrpg_global&channel_id=1&game=hkrpg&game_biz=hkrpg_global&lang=en&level=70&platform=pc&region=prod_official_asia&uid=1
#[derive(Deserialize, Clone)]
pub struct GetAlertPicReq {
    pub bundle_id: String,
    pub channel_id: i32,
    pub game: String,
    pub game_biz: String,
    pub lang: String,
    pub level: u32,
    pub platform: String,
    pub region: String,
    pub uid: u32,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Query
// URL: http://localhost:.../my_register
#[derive(Deserialize, Clone)]
pub struct MyRegisterReq {
    pub username: String,
    pub password: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// URL: https://api-account-os.hoyoverse.com/account/risky/api/check?
#[derive(Deserialize, Clone)]
pub struct RiskyApiCheckReq {
    pub action_type: String,
    pub api_name: String,
    pub username: String,
}

// ------------------------------------------------------------------------- //

// Request Type: POST
// Parse Target: Body
// Note: to decrypt password needs patch. shitfix is using username box for username and password.
// URL: https://hkrpg-sdk-os.hoyoverse.com/hkrpg_global/mdk/shield/api/login
#[derive(Deserialize, Clone)]
pub struct ShieldApiLoginReq {
    pub account: String,
    pub password: String,
    pub is_crypto: bool,
}

// ------------------------------------------------------------------------- //
