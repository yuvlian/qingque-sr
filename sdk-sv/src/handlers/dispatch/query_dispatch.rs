use crate::ArcState;
use crate::app::request::QueryDispatchReq;
use axum::extract::{Query, State};
use sr_proto::{Dispatch, Message, RegionInfo};

pub async fn get(State(state): State<ArcState>, Query(_query): Query<QueryDispatchReq>) -> String {
    let rsp = Dispatch {
        msg: "OK".into(),
        top_sever_region_name: "stalel".into(),
        region_list: vec![RegionInfo {
            name: "stalel".into(),
            title: "stalel".into(),
            dispatch_url: format!(
                "{}:{}/query_gateway",
                state.env.sdk_sv_host, state.env.sdk_sv_port
            ),
            env_type: state.env.login_env_type.clone(),
            display_name: "stalel".into(),
            msg: "OK".into(),
        }],
        ..Default::default()
    };

    rbase64::encode(&rsp.encode_to_vec())
}
