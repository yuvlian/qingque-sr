package srtools

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/yuvlian/qingque-sr/pb"
)

func (s *SRToolsLite) GetBattleID() uint32 {
	return s.BattleConfig.BattleID
}

func (s *SRToolsLite) GetStageID() uint32 {
	return s.BattleConfig.StageID
}

func (s *SRToolsLite) GetCycleCount() uint32 {
	return s.BattleConfig.CycleCount
}

func (s *SRToolsLite) GetBattleBuffs() []*pb.BattleBuff {
	totalBuffs := 0
	for _, av := range s.AvatarConfig {
		totalBuffs += len(av.BuffIDList)
	}
	buffs := make([]*pb.BattleBuff, 0, totalBuffs)

	for i, av := range s.AvatarConfig {
		for _, buffID := range av.BuffIDList {
			dynamicValues := map[string]float32{
				"SkillIndex": 0.0,
			}

			// march 7th hunt hardcode
			if isMarchHuntBuffID(buffID) {
				dynamicValues["#ADF_1"] = 3.0
				dynamicValues["#ADF_2"] = 3.0
			}

			buff := &pb.BattleBuff{
				Id:              buffID,
				OwnerIndex:      uint32(i),
				Level:           1,
				WaveFlag:        ^uint32(0),
				TargetIndexList: []uint32{0, 1, 2, 3, 4},
				DynamicValues:   dynamicValues,
			}

			buffs = append(buffs, buff)
		}
	}

	return buffs
}

func (s *SRToolsLite) GetBattleWaves() []*pb.SceneMonsterWave {
	waves := make([]*pb.SceneMonsterWave, 0, len(s.BattleConfig.MonsterWave))

	for _, monsterIDs := range s.BattleConfig.MonsterWave {
		monsters := make([]*pb.SceneMonster, 0, len(monsterIDs))
		for _, monsterID := range monsterIDs {
			monsters = append(monsters, &pb.SceneMonster{
				MonsterId: monsterID,
			})
		}

		wave := &pb.SceneMonsterWave{
			MonsterList: monsters,
			MonsterParam: &pb.SceneMonsterWaveParam{
				Level: s.BattleConfig.MonsterLevel,
			},
		}

		waves = append(waves, wave)
	}

	return waves
}

func buildTrace(timedID uint32, entries ...[2]uint32) []*pb.AvatarSkillTree {
	trace := make([]*pb.AvatarSkillTree, 0, len(entries))
	for _, entry := range entries {
		pointOffset := entry[0]
		level := entry[1]
		trace = append(trace, &pb.AvatarSkillTree{
			PointId: timedID + pointOffset,
			Level:   level,
		})
	}
	return trace
}

// / this doesn't care about eidolon.
func (a *AvatarConfig) GetMaxedTrace() []*pb.AvatarSkillTree {
	avatarID := a.ID
	timedID := avatarID * 1000

	if RemembranceIDs[avatarID] {
		return buildTrace(timedID,
			[2]uint32{1, 6},
			[2]uint32{2, 10},
			[2]uint32{3, 10},
			[2]uint32{4, 10},
			[2]uint32{7, 1},
			[2]uint32{101, 1},
			[2]uint32{102, 1},
			[2]uint32{103, 1},
			[2]uint32{201, 1},
			[2]uint32{202, 1},
			[2]uint32{203, 1},
			[2]uint32{204, 1},
			[2]uint32{205, 1},
			[2]uint32{206, 1},
			[2]uint32{207, 1},
			[2]uint32{208, 1},
			[2]uint32{209, 1},
			[2]uint32{210, 1},
			[2]uint32{301, 5},
			[2]uint32{302, 5},
		)
	} else if EnhancedIDs[avatarID] {
		return buildTrace(timedID,
			[2]uint32{1 + 10000000, 6},
			[2]uint32{2 + 10000000, 10},
			[2]uint32{3 + 10000000, 10},
			[2]uint32{4 + 10000000, 10},
			[2]uint32{7 + 10000000, 1},
			[2]uint32{101 + 10000000, 1},
			[2]uint32{102 + 10000000, 1},
			[2]uint32{103 + 10000000, 1},
			[2]uint32{201 + 10000000, 1},
			[2]uint32{202 + 10000000, 1},
			[2]uint32{203 + 10000000, 1},
			[2]uint32{204 + 10000000, 1},
			[2]uint32{205 + 10000000, 1},
			[2]uint32{206 + 10000000, 1},
			[2]uint32{207 + 10000000, 1},
			[2]uint32{208 + 10000000, 1},
			[2]uint32{209 + 10000000, 1},
			[2]uint32{210 + 10000000, 1},
		)
	} else {
		return buildTrace(timedID,
			[2]uint32{1, 6},
			[2]uint32{2, 10},
			[2]uint32{3, 10},
			[2]uint32{4, 10},
			[2]uint32{7, 1},
			[2]uint32{101, 1},
			[2]uint32{102, 1},
			[2]uint32{103, 1},
			[2]uint32{201, 1},
			[2]uint32{202, 1},
			[2]uint32{203, 1},
			[2]uint32{204, 1},
			[2]uint32{205, 1},
			[2]uint32{206, 1},
			[2]uint32{207, 1},
			[2]uint32{208, 1},
			[2]uint32{209, 1},
			[2]uint32{210, 1},
		)
	}
}

func parseUint32(s string) (uint32, error) {
	val, err := strconv.ParseUint(strings.TrimSpace(s), 10, 32)
	return uint32(val), err
}

func parseSubAffix(s string) ([3]uint32, bool) {
	parts := strings.Split(s, ":")
	if len(parts) != 2 && len(parts) != 3 {
		return [3]uint32{}, false
	}

	id, err1 := parseUint32(parts[0])
	cnt, err2 := parseUint32(parts[1])
	if err1 != nil || err2 != nil {
		return [3]uint32{}, false
	}

	var step uint32
	if len(parts) == 3 {
		val, err := parseUint32(parts[2])
		if err != nil {
			return [3]uint32{}, false
		}
		step = val
	} else {
		step = cnt * 2
	}

	return [3]uint32{id, cnt, step}, true
}

func parseRelicString(s string) (uint32, uint32, uint32, uint32, [3]uint32, [3]uint32, [3]uint32, [3]uint32, bool) {
	parts := strings.Split(s, ",")
	if len(parts) < 8 {
		return 0, 0, 0, 0, [3]uint32{}, [3]uint32{}, [3]uint32{}, [3]uint32{}, false
	}

	relicID, err1 := parseUint32(parts[0])
	relicLv, err2 := parseUint32(parts[1])
	relicMAffix, err3 := parseUint32(parts[2])
	relicTotalSAffix, err4 := parseUint32(parts[3])
	if err1 != nil || err2 != nil || err3 != nil || err4 != nil {
		return 0, 0, 0, 0, [3]uint32{}, [3]uint32{}, [3]uint32{}, [3]uint32{}, false
	}

	saffix1, ok1 := parseSubAffix(parts[4])
	saffix2, ok2 := parseSubAffix(parts[5])
	saffix3, ok3 := parseSubAffix(parts[6])
	saffix4, ok4 := parseSubAffix(parts[7])
	if !ok1 || !ok2 || !ok3 || !ok4 {
		return 0, 0, 0, 0, [3]uint32{}, [3]uint32{}, [3]uint32{}, [3]uint32{}, false
	}

	return relicID, relicLv, relicMAffix, relicTotalSAffix, saffix1, saffix2, saffix3, saffix4, true
}

func (a *AvatarConfig) GetBattleRelics() ([]*pb.BattleRelic, []string) {
	relicStrings := a.Relics
	avatarID := a.ID
	relics := make([]*pb.BattleRelic, 0, len(relicStrings))
	var errors []string

	for _, rstring := range relicStrings {
		relicID, relicLv, relicMAffix, _, saffix1, saffix2, saffix3, saffix4, ok := parseRelicString(rstring)
		if !ok {
			errors = append(errors, fmt.Sprintf("invalid relic string for %d: %s", avatarID, rstring))
			continue
		}

		subAffixList := []*pb.RelicAffix{
			{AffixId: saffix1[0], Cnt: saffix1[1], Step: saffix1[2]},
			{AffixId: saffix2[0], Cnt: saffix2[1], Step: saffix2[2]},
			{AffixId: saffix3[0], Cnt: saffix3[1], Step: saffix3[2]},
			{AffixId: saffix4[0], Cnt: saffix4[1], Step: saffix4[2]},
		}

		relics = append(relics, &pb.BattleRelic{
			Id:           relicID,
			Level:        relicLv,
			MainAffixId:  relicMAffix,
			SubAffixList: subAffixList,
		})
	}

	return relics, errors
}

func (a *AvatarConfig) GetEnergy() *pb.SpBarInfo {
	return &pb.SpBarInfo{
		CurSp: a.SP * 100,
		MaxSp: 10000,
	}
}

func (s *SRToolsLite) GetBattleAvatars() ([]*pb.BattleAvatar, []string) {
	battleAvatars := make([]*pb.BattleAvatar, 0, len(s.AvatarConfig))
	var errors []string

	for i, avatar := range s.AvatarConfig {
		traces := avatar.GetMaxedTrace()

		relics, relicErrors := avatar.GetBattleRelics()
		if len(relicErrors) > 0 {
			errors = append(errors, relicErrors...)
		}

		spBar := avatar.GetEnergy()

		lightCone := &pb.BattleEquipment{
			Id:        avatar.LightCone.ID,
			Rank:      avatar.LightCone.Rank,
			Level:     avatar.LightCone.Level,
			Promotion: avatar.LightCone.Promotion,
		}

		battleAvatar := &pb.BattleAvatar{
			EnhancedId:    0,
			AvatarType:    pb.AvatarType_AVATAR_FORMAL_TYPE,
			Id:            avatar.ID,
			Level:         avatar.Level,
			Rank:          avatar.Rank,
			Index:         uint32(i),
			SkilltreeList: traces,
			EquipmentList: []*pb.BattleEquipment{lightCone},
			Hp:            avatar.HP * 100,
			SpBar:         spBar,
			Promotion:     avatar.Promotion,
			RelicList:     relics,
		}

		if EnhancedIDs[avatar.ID] {
			battleAvatar.EnhancedId = 1
		}

		battleAvatars = append(battleAvatars, battleAvatar)
	}

	return battleAvatars, errors
}

func createDefault() SRToolsLite {
	return SRToolsLite{
		AvatarConfig: []AvatarConfig{
			{
				ID:        1014,
				HP:        100,
				SP:        50,
				Level:     80,
				Promotion: 6,
				Rank:      0,
				LightCone: LightCone{
					ID:        24000,
					Rank:      5,
					Level:     80,
					Promotion: 6,
				},
				Relics: []string{
					"61221,15,1,4,2:1:2,5:1:2,7:1:2,9:5:10",
					"61222,15,1,4,5:1:2,7:1:2,8:1:2,9:5:10",
					"61223,15,4,4,2:1:2,5:1:2,7:1:2,9:5:10",
					"61224,15,4,4,2:1:2,5:5:10,8:1:2,9:1:2",
					"63065,15,8,4,4:3:6,6:1:2,11:3:6,12:1:2",
					"63066,15,4,4,7:1:2,9:2:4,10:1:2,11:5:10",
				},
				BuffIDList: []uint32{101401, 1000115},
			},
			{
				ID:        1202,
				HP:        100,
				SP:        100,
				Level:     80,
				Promotion: 6,
				Rank:      6,
				LightCone: LightCone{
					ID:        20012,
					Rank:      5,
					Level:     80,
					Promotion: 6,
				},
				Relics: []string{
					"61211,15,1,4,5:1:2,6:6:12,7:1:2,11:1:2",
					"61212,15,1,4,5:1:2,6:1:2,7:6:12,11:1:2",
					"61213,15,2,4,2:1:2,4:1:2,6:1:2,11:6:12",
					"61214,15,4,4,4:6:12,5:1:2,6:1:2,11:1:2",
					"63175,15,2,4,1:1:2,3:1:2,6:6:12,11:1:2",
					"63176,15,2,4,1:1:2,3:1:2,6:6:12,11:1:2",
				},
				BuffIDList: []uint32{},
			},
			{
				ID:        1101,
				HP:        100,
				SP:        50,
				Level:     80,
				Promotion: 6,
				Rank:      0,
				LightCone: LightCone{
					ID:        21011,
					Rank:      5,
					Level:     80,
					Promotion: 6,
				},
				Relics: []string{
					"61211,15,1,4,6:1:2,7:6:12,9:1:2,11:1:2",
					"61212,15,1,4,6:1:2,7:6:12,9:1:2,11:1:2",
					"61213,15,5,4,4:1:2,6:1:2,7:6:12,11:1:2",
					"61214,15,4,4,4:1:2,6:1:2,9:6:12,11:1:2",
					"63175,15,3,4,4:1:2,7:1:2,9:6:12,11:1:2",
					"63176,15,2,4,4:1:2,7:1:2,9:6:12,11:1:2",
				},
				BuffIDList: []uint32{110101},
			},
			{
				ID:        1217,
				HP:        100,
				SP:        50,
				Level:     80,
				Promotion: 6,
				Rank:      0,
				LightCone: LightCone{
					ID:        21000,
					Rank:      5,
					Level:     80,
					Promotion: 6,
				},
				Relics: []string{
					"61211,15,1,4,3:1:2,4:1:2,7:1:2,11:6:12",
					"61212,15,1,4,3:1:2,4:1:2,7:1:2,11:6:12",
					"61213,15,6,4,3:1:2,4:1:2,7:1:2,11:6:12",
					"61214,15,4,4,1:1:2,3:1:2,4:6:12,11:1:2",
					"63175,15,1,4,1:6:12,3:1:2,6:1:2,7:1:2",
					"63176,15,2,4,3:1:2,4:6:12,6:1:2,7:1:2",
				},
				BuffIDList: []uint32{121701},
			},
		},
		BattleConfig: BattleConfig{
			BattleID:   1,
			StageID:    30114122,
			CycleCount: 30,
			MonsterWave: [][]uint32{
				{4033010, 4033030, 4032030},
				{2034010},
			},
			MonsterLevel: 95,
		},
	}
}
