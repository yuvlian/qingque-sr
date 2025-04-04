use crate::app::{
    request::GetAnnListReq,
    response::{
        Announcement, AnnouncementCategory, AnnouncementType, GetAnnListRsp, IRsp, PictureCategory,
    },
};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(query): Query<GetAnnListReq>) -> Json<IRsp<GetAnnListRsp>> {
    Json(IRsp::<GetAnnListRsp> {
        data: Some(GetAnnListRsp {
            list: vec![AnnouncementCategory {
                list: vec![Announcement {
                    ann_id: 186,
                    title: "Honkai: Star Rail Fair Gaming Declaration".to_string(),
                    subtitle: "Honkai: Star Rail Fair Gaming Declaration".to_string(),
                    banner: "https://codeberg.org/yuvlian/cdn/raw/branch/main/pps.png".to_string(),
                    type_label: "Notices".to_string(),
                    tag_label: "重要".to_string(),
                    tag_icon: "https://codeberg.org/yuvlian/cdn/raw/branch/main/3234.png"
                        .to_string(),
                    login_alert: 1,
                    lang: query.lang,
                    start_time: "2023-04-04 00:00:00".to_string(),
                    end_time: "2035-06-07 06:00:00".to_string(),
                    r#type: 4,
                    tag_start_time: "2000-01-02 15:04:05".to_string(),
                    tag_end_time: "2030-01-02 15:04:05".to_string(),
                    remind_ver: 1,
                    has_content: true,
                    logout_remind_ver: 1,
                    ..Default::default()
                }],
                type_id: 4,
                type_label: "Notices".to_string(),
            }],
            total: 1,
            type_list: vec![AnnouncementType {
                id: 4,
                name: "公告".to_string(),
                mi18n_name: "Notices".to_string(),
            }],
            timezone: 8,
            t: "1742222317_ae6295e8279662a881eb9beb19144112_d41d8cd98f00b204e9800998ecf8427e"
                .to_string(),
            pic_list: vec![PictureCategory {
                type_id: 3,
                type_label: "News".to_string(),
                ..Default::default()
            }],
            pic_type_list: vec![AnnouncementType {
                id: 3,
                name: "资讯".to_string(),
                mi18n_name: "News".to_string(),
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
}
