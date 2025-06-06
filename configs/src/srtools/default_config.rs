Self {
    avatar_config: vec![
        AvatarConfig {
            // name: String::from("Bronya"),
            id: 1101,
            hp: 100,
            sp: 50,
            level: 80,
            promotion: 6,
            rank: 0,
            lightcone: LightCone {
                id: 23003,
                rank: 1,
                level: 80,
                promotion: 6,
            },
            relics: vec![
                String::from("61041,15,1,4,3:1:2,6:1:2,7:6:12,11:1:2"),
                String::from("61042,15,1,4,3:1:2,6:1:2,9:6:12,11:1:2"),
                String::from("61043,15,5,4,1:1:2,3:1:2,6:1:2,11:6:12"),
                String::from("61044,15,3,4,1:6:12,3:1:2,4:1:2,11:1:2"),
                String::from("63105,15,1,4,1:3:6,3:1:2,6:1:2,7:4:8"),
                String::from("63106,15,2,4,1:1:2,3:1:2,7:1:2,11:5:0"),
            ],
            // use_technique: true,
            buff_id_list: vec![110101],
        },
        AvatarConfig {
            // name: String::from("Fu Xuan"),
            id: 1208,
            hp: 100,
            sp: 50,
            level: 80,
            promotion: 6,
            rank: 0,
            lightcone: LightCone {
                id: 23005,
                rank: 1,
                level: 80,
                promotion: 6,
            },
            relics: vec![
                String::from("61131,15,1,4,3:1:2,4:5:10,7:1:2,8:1:2"),
                String::from("61132,15,1,4,3:1:2,4:5:10,7:1:2,8:1:2"),
                String::from("61133,15,3,4,3:1:2,4:5:10,7:1:2,8:1:2"),
                String::from("61134,15,4,4,3:1:2,4:5:10,6:1:2,8:1:2"),
                String::from("63105,15,1,4,1:5:10,3:1:2,6:1:2,8:1:2"),
                String::from("63106,15,2,4,1:1:2,4:1:2,7:1:2,11:6:12"),
            ],
            // use_technique: true,
            buff_id_list: vec![120801, 120802],
        },
        AvatarConfig {
            // name: String::from("Ruan Mei"),
            id: 1303,
            hp: 100,
            sp: 50,
            level: 80,
            promotion: 6,
            rank: 0,
            lightcone: LightCone {
                id: 20012,
                rank: 5,
                level: 80,
                promotion: 6,
            },
            relics: vec![
                String::from("61181,15,1,4,3:1:2,6:5:10,7:1:2,11:1:2"),
                String::from("61182,15,1,4,3:1:2,6:5:10,7:1:2,11:1:2"),
                String::from("61183,15,1,4,3:1:2,6:5:10,7:1:2,11:1:2"),
                String::from("61184,15,1,4,3:1:2,6:5:10,7:1:2,11:1:2"),
                String::from("63085,15,1,4,3:1:2,6:1:2,7:1:2,11:6:12"),
                String::from("63086,15,2,4,3:1:2,6:1:2,7:1:2,11:6:12"),
            ],
            // use_technique: true,
            buff_id_list: vec![130301, 130302, 130303],
        },
        AvatarConfig {
            // name: String::from("Qingque"),
            id: 1201,
            hp: 100,
            sp: 50,
            level: 80,
            promotion: 6,
            rank: 6,
            lightcone: LightCone {
                id: 21034,
                rank: 5,
                level: 80,
                promotion: 6,
            },
            relics: vec![
                String::from("61081,15,1,4,2:1:0,5:1:0,8:1:0,9:5:0"),
                String::from("61082,15,1,4,3:1:2,5:1:0,8:1:0,9:5:0"),
                String::from("61083,15,4,4,1:1:2,3:1:2,5:1:0,9:5:0"),
                String::from("61084,15,4,4,1:3:6,3:1:2,5:1:0,8:4:0"),
                String::from("63095,15,2,4,2:2:4,3:3:6,8:2:2,9:2:2"),
                String::from("63096,15,4,4,2:1:2,7:1:2,8:5:10,9:1:2"),
            ],
            // use_technique: true,
            buff_id_list: vec![120101, 1000116],
        },
    ],
    battle_config: BattleConfig {
        battle_id: 1,
        stage_id: 30106111,
        cycle_count: 30,
        monster_wave: vec![
            vec![1023020, 8003010],
            vec![100401010, 1023020],
        ],
        monster_level: 92,
        // blessings: vec![],
    },
}
