use sqlx::{FromRow, SqlitePool};
use sr_proto::{GateServer, Message};

#[derive(FromRow, Debug, Default)]
pub struct GatewayHotfix {
    pub game_version: String,
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub ifix_url: String,
}

// thx amizing
impl GatewayHotfix {
    pub fn parse_version_string(ver: &str) -> (Option<&str>, Option<&str>, Option<&str>) {
        const REGIONS: [&str; 2] = ["CN", "OS"];
        const OS_LIST: [&str; 3] = ["Android", "Win", "iOS"];

        let (region, rest) = REGIONS
            .iter()
            .find_map(|&r| ver.strip_prefix(r).map(|rest| (Some(r), rest)))
            .unwrap_or((None, ver));

        let os = OS_LIST
            .iter()
            .find_map(|&os| rest.find(os).map(|pos| (os, pos)));

        let (os_name, branch) = match os {
            Some((os, os_pos)) => {
                let branch = rest[..os_pos].trim();
                (
                    Some(os),
                    if branch.is_empty() {
                        None
                    } else {
                        Some(branch)
                    },
                )
            }
            None => (None, None),
        };

        (region, branch, os_name)
    }

    pub async fn fetch_hotfix(
        ver: &str,
        dispatch_seed: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        const CNPROD_HOST_WIN: &str = "prod-gf-cn-dp01.bhsr.com";
        const CNBETA_HOST_WIN: &str = "beta-release01-cn.bhsr.com";
        const OSPROD_HOST_WIN: &str = "prod-official-asia-dp01.starrails.com";
        const OSBETA_HOST_WIN: &str = "beta-release01-asia.starrails.com";
        const NEON_PROXY: &str = "proxy1.neonteam.dev";

        let ver_parsed = Self::parse_version_string(ver);
        let region = ver_parsed.0.ok_or("Region not found in version string")?;
        let branch = ver_parsed.1.ok_or("Branch not found in version string")?;
        let os = ver_parsed.2.ok_or("OS not found in version string")?;

        let host = match (region, branch, os) {
            ("OS", "BETA", "Win") => Some(OSBETA_HOST_WIN),
            ("OS", "PROD", "Win") => Some(OSPROD_HOST_WIN),
            ("CN", "BETA", "Win") => Some(CNBETA_HOST_WIN),
            ("CN", "PROD", "Win") => Some(CNPROD_HOST_WIN),
            _ => None,
        }
        .ok_or("No matching host for provided version")?;

        let url = format!(
            "https://{}/{}/query_gateway?version={}&platform_type=1&language_type=3&dispatch_seed={}&channel_id=1&sub_channel_id=1&is_need_url=1",
            NEON_PROXY, host, ver, dispatch_seed
        );

        tracing::info!("Auto Hotfix: {}", url);

        let res = reqwest::get(url)
            .await
            .map_err(|e| format!("Request failed: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        let bytes = rbase64::decode(&res).map_err(|e| format!("Base64 decode failed: {}", e))?;
        let decoded = GateServer::decode(bytes.as_slice())
            .map_err(|e| format!("Failed to decode GateServer: {}", e))?;

        if decoded.retcode != 0 || res.is_empty() {
            return Err("Invalid response received".into());
        }

        let gateway_hotfix = Self {
            game_version: ver.into(),
            asset_bundle_url: decoded.asset_bundle_url,
            ex_resource_url: decoded.ex_resource_url,
            lua_url: decoded.lua_url,
            ifix_url: decoded.ifix_url,
        };

        tracing::info!("Obtained Hotfix: {:?}", gateway_hotfix);

        Ok(gateway_hotfix)
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO gateway_hotfix (game_version, asset_bundle_url, ex_resource_url, lua_url, ifix_url) 
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&self.game_version)
        .bind(&self.asset_bundle_url)
        .bind(&self.ex_resource_url)
        .bind(&self.lua_url)
        .bind(&self.ifix_url)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_or_fetch(
        pool: &SqlitePool,
        ver: &str,
        dispatch_seed: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(v) = Self::get_by_version(pool, ver).await? {
            return Ok(v);
        }

        match Self::fetch_hotfix(ver, dispatch_seed).await {
            Ok(v) => {
                v.insert(pool).await?;
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_by_version(
        pool: &SqlitePool,
        version: &str,
    ) -> Result<Option<Self>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sqlx::query_as::<_, GatewayHotfix>(
            "SELECT game_version, asset_bundle_url, ex_resource_url, lua_url, ifix_url 
             FROM gateway_hotfix WHERE game_version = ?",
        )
        .bind(version)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}
