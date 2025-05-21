use serde::Deserialize;
use sr_proto::{
    Avatar, AvatarType, Gender, LineupAvatar, LineupInfo, MultiPathAvatarType, SpBarInfo,
};
use tokio::fs;

type MarchPath = MultiPathAvatarType;
type TrailblazerPath = MultiPathAvatarType;

#[derive(Deserialize)]
pub struct AvatarConfig {
    lineup: Vec<u32>,
    trailblazer_gender: String,
    march_element: String,
    trailblazer_element: String,
    owned_avatars: Vec<u32>,
}

impl Default for AvatarConfig {
    fn default() -> Self {
        Self {
            lineup: vec![1201, 1001, 8001],
            trailblazer_gender: String::from("Man"),
            march_element: String::from("Imaginary"),
            trailblazer_element: String::from("Imaginary"),
            owned_avatars: vec![1201, 1001, 8001],
        }
    }
}

impl AvatarConfig {
    pub async fn from_file(file_path: &str) -> Self {
        toml::from_str(&fs::read_to_string(file_path).await.unwrap_or_default()).unwrap_or_default()
    }

    pub fn get_trailblazer_gender(&self) -> Gender {
        match self.trailblazer_gender.as_str() {
            "Woman" => Gender::Woman,
            "Man" => Gender::Man,
            _ => Gender::None,
        }
    }

    pub fn get_cur_lineup(&self) -> LineupInfo {
        let av_list: Vec<LineupAvatar> = self
            .lineup
            .iter()
            .enumerate()
            .map(|(i, id)| LineupAvatar {
                id: *id,
                hp: 10000,
                slot: i as u32,
                satiety: 0,
                sp_bar: Some(SpBarInfo {
                    cur_sp: 0,
                    max_sp: 10000,
                }),
                avatar_type: AvatarType::AvatarFormalType.into(),
            })
            .collect();

        LineupInfo {
            name: String::from("yulianteam"),
            avatar_list: av_list,
            plane_id: 20101,
            ..Default::default()
        }
    }

    pub fn get_owned_avatars(&self) -> Vec<Avatar> {
        self.owned_avatars
            .iter()
            .map(|id| Avatar {
                promotion: 6,
                rank: 6,
                exp: 0,
                level: 80,
                base_avatar_id: *id,
                ..Default::default()
            })
            .collect()
    }

    pub fn get_multipath_data(&self, gender: Gender) -> (MarchPath, TrailblazerPath) {
        let march = self.march_element.as_str();
        let tb = self.trailblazer_element.as_str();

        let march_path = match march {
            "Imaginary" => MultiPathAvatarType::Mar7thRogueType,
            "Ice" => MultiPathAvatarType::Mar7thKnightType,
            _ => MultiPathAvatarType::None,
        };

        let tb_path = match (tb, gender) {
            ("Imaginary", Gender::Woman) => MultiPathAvatarType::GirlShamanType,
            ("Imaginary", Gender::Man) => MultiPathAvatarType::BoyShamanType,
            ("Fire", Gender::Woman) => MultiPathAvatarType::GirlKnightType,
            ("Fire", Gender::Man) => MultiPathAvatarType::BoyKnightType,
            ("Physical", Gender::Woman) => MultiPathAvatarType::GirlWarriorType,
            ("Physical", Gender::Man) => MultiPathAvatarType::BoyWarriorType,
            ("Ice", Gender::Woman) => MultiPathAvatarType::GirlMemoryType,
            ("Ice", Gender::Man) => MultiPathAvatarType::BoyMemoryType,
            _ => MultiPathAvatarType::None,
        };

        (march_path, tb_path)
    }
}
