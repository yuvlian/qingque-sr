use crate::app::{
    request::DeviceGetExtListReq,
    response::{DeviceGetExtListRsp, IRsp},
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(_query): Query<DeviceGetExtListReq>) -> Json<IRsp<DeviceGetExtListRsp>> {
    Json(IRsp::<DeviceGetExtListRsp> {
        data: Some(DeviceGetExtListRsp {
            code: 200,
            msg: "ok".to_string(),
            pkg_str: "/vK5WTh5SS3SAj8Zm0qPWg==".to_string(),
            ext_list: vec![
                "cpuName".to_string(),
                "deviceModel".to_string(),
                "deviceName".to_string(),
                "deviceType".to_string(),
                "deviceUID".to_string(),
                "gpuID".to_string(),
                "gpuName".to_string(),
                "gpuAPI".to_string(),
                "gpuVendor".to_string(),
                "gpuVersion".to_string(),
                "gpuMemory".to_string(),
                "osVersion".to_string(),
                "cpuCores".to_string(),
                "cpuFrequency".to_string(),
                "gpuVendorID".to_string(),
                "isGpuMultiTread".to_string(),
                "memorySize".to_string(),
                "screenSize".to_string(),
                "engineName".to_string(),
                "addressMAC".to_string(),
                "packageVersion".to_string(),
            ],
            ..Default::default()
        }),
        ..Default::default()
    })
}
