use crate::ArcState;
use crate::app::request::QueryDispatchReq;
use axum::extract::{Query, State};
use sr_proto::{Dispatch, Message, RegionInfo};

pub async fn get(State(state): State<ArcState>, Query(_query): Query<QueryDispatchReq>) -> String {
    let rsp = Dispatch {
        msg: "OK".to_string(),
        top_sever_region_name: "stalel".to_string(),
        region_list: vec![RegionInfo {
            name: "stalel".to_string(),
            title: "stalel".to_string(),
            dispatch_url: format!(
                "http://{}:{}/query_gateway",
                state.env.sdk_sv_host, state.env.sdk_sv_port
            ),
            env_type: state.env.login_env_type.clone(),
            display_name: "stalel".to_string(),
            msg: "OK".to_string(),
        }],
        ..Default::default()
    };

    rbase64::encode(&rsp.encode_to_vec())
}
