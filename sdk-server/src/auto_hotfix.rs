use configs::hotfix::{GameVersion, HotfixConfig};
use sr_proto::GateServer;
use sr_proto::prost::Message;

const PROXY_HOST: &str = "proxy1.neonteam.dev";
const CN_PROD_HOST: &str = "prod-gf-cn-dp01.bhsr.com";
const CN_BETA_HOST: &str = "beta-release01-cn.bhsr.com";
const OS_PROD_HOST: &str = "prod-official-asia-dp01.starrails.com";
const OS_BETA_HOST: &str = "beta-release01-asia.starrails.com";

pub async fn try_get_and_update(version: &str, dispatch_seed: &str) -> Result<GameVersion, String> {
    let mut current_hotfix = GameVersion::from_file("_configs_/hotfix.json").await;

    if let Some(_) = current_hotfix.0.get(version) {
        tracing::info!("hotfix already exists. no need autohotfix.");
        return Ok(current_hotfix);
    }

    let host = match version {
        v if v.starts_with("CNPROD") => CN_PROD_HOST,
        v if v.starts_with("CNBETA") => CN_BETA_HOST,
        v if v.starts_with("OSPROD") => OS_PROD_HOST,
        v if v.starts_with("OSBETA") => OS_BETA_HOST,
        _ => return Err(format!("invalid version: {}", version)),
    };

    let target = format!(
        "https://{}/{}/query_gateway?version={}&dispatch_seed={}&language_type=1&platform_type=2&channel_id=1&sub_channel_id=1&is_need_url=1&account_type=1",
        PROXY_HOST, host, version, dispatch_seed
    );

    tracing::info!("auto hotfix target url: {}", &target);

    let base64_rsp = match reqwest::get(&target).await {
        Ok(v) => v.text().await.unwrap(),
        Err(e) => return Err(e.to_string()),
    };

    let base64_decoded = rbase64::decode(&base64_rsp).unwrap();
    let gateserver = GateServer::decode(base64_decoded.as_ref()).unwrap_or_default();

    if gateserver.asset_bundle_url.is_empty() && gateserver.ex_resource_url.is_empty() {
        return Err(String::from("asset bundle and ex resource url is empty"));
    }

    tracing::info!("auto hotfix for {} success, updating hotfix.json", version);
    tracing::info!(
        "\nasset_bundle_url: {}\nex_resource_url: {}\nifix_url: {}\nlua_url: {}\nlua_version: {}",
        &gateserver.asset_bundle_url,
        &gateserver.ex_resource_url,
        &gateserver.ifix_url,
        &gateserver.lua_url,
        &gateserver.mdk_res_version,
    );

    let new_config = HotfixConfig {
        asset_bundle_url: gateserver.asset_bundle_url,
        ex_resource_url: gateserver.ex_resource_url,
        ifix_url: gateserver.ifix_url,
        lua_url: gateserver.lua_url,
        lua_version: gateserver.mdk_res_version,
    };

    current_hotfix.insert_hotfix_by_version(version, new_config);

    if let Err(e) = current_hotfix.save_to_file("_configs_/hotfix.json").await {
        tracing::warn!(
            "failed saving new hotfix to file: {}, you can save it manually.",
            e
        );
    }

    Ok(current_hotfix)
}
