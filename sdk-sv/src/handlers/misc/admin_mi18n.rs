use crate::app::response::AdminMi18nPlatRsp;
use axum::extract::OriginalUri;
use axum::response::Json;

pub async fn get(OriginalUri(uri): OriginalUri) -> Json<AdminMi18nPlatRsp> {
    let url = format!("https://proxy1.neonteam.dev/webstatic.hoyoverse.com{}", uri);

    match reqwest::get(url).await {
        Ok(v) => match v.json::<AdminMi18nPlatRsp>().await {
            Ok(t) => Json(t),
            _ => {
                return Json(AdminMi18nPlatRsp::default());
            }
        },
        _ => {
            return Json(AdminMi18nPlatRsp::default());
        }
    }
}
