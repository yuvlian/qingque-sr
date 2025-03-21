use sqlx::{Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct GatewayHotfix {
    pub game_version: String,
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub ifix_url: String,
}

impl GatewayHotfix {
    pub async fn fetch(dispatch_seed: &str) -> Self {
        // use single connection reqwest (no need client, overkill methinks)
        // fetch url, decode with proto, insert db, yada yada...
        todo!()
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO GatewayHotfix (version, asset_bundle_url, ex_resource_url, lua_url, ifix_url) 
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

    pub async fn get_by_version(pool: &SqlitePool, version: &str) -> Result<Option<Self>, Error> {
        let result = sqlx::query_as::<_, GatewayHotfix>(
            "SELECT game_version, asset_bundle_url, ex_resource_url, lua_url, ifix_url 
             FROM GatewayHotfix WHERE game_version = ?",
        )
        .bind(version)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}
