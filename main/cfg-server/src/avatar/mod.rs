use net_msg::pb::{
    AmountInfo, Avatar, AvatarType, Gender, LineupAvatar, LineupInfo, MultiPathAvatarType,
};
use serde::Deserialize;
use std::fs;

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
    pub fn from_file(file_path: &str) -> Self {
        let avatar_toml_data = fs::read_to_string(file_path);
        match avatar_toml_data {
            Ok(data) => toml::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
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
                slot_type: i as u32,
                satiety: 0,
                sp: Some(AmountInfo {
                    cur_amount: 0,
                    max_amount: 10000,
                }),
                avatar_type: AvatarType::AvatarFormalType.into(),
            })
            .collect();

        LineupInfo {
            name: String::from("smolteam"),
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

        match (march, tb, gender) {
            ("Imaginary", "Imaginary", Gender::Woman) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::GirlShamanType,
            ),
            ("Imaginary", "Imaginary", Gender::Man) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::BoyShamanType,
            ),
            ("Imaginary", "Fire", Gender::Woman) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::GirlKnightType,
            ),
            ("Imaginary", "Fire", Gender::Man) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::BoyKnightType,
            ),
            ("Imaginary", "Physical", Gender::Woman) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::GirlWarriorType,
            ),
            ("Imaginary", "Physical", Gender::Man) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::BoyWarriorType,
            ),
            ("Imaginary", "Ice", Gender::Woman) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::GirlMemoryType,
            ),
            ("Imaginary", "Ice", Gender::Man) => (
                MultiPathAvatarType::Mar7thRogueType,
                MultiPathAvatarType::BoyMemoryType,
            ),
            ("Ice", "Imaginary", Gender::Woman) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::GirlShamanType,
            ),
            ("Ice", "Imaginary", Gender::Man) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::BoyShamanType,
            ),
            ("Ice", "Fire", Gender::Woman) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::GirlKnightType,
            ),
            ("Ice", "Fire", Gender::Man) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::BoyKnightType,
            ),
            ("Ice", "Physical", Gender::Woman) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::GirlWarriorType,
            ),
            ("Ice", "Physical", Gender::Man) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::BoyWarriorType,
            ),
            ("Ice", "Ice", Gender::Woman) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::GirlMemoryType,
            ),
            ("Ice", "Ice", Gender::Man) => (
                MultiPathAvatarType::Mar7thKnightType,
                MultiPathAvatarType::BoyMemoryType,
            ),
            _ => (MultiPathAvatarType::None, MultiPathAvatarType::None),
        }
    }
}
