package player

import "github.com/yuvlian/qingque-sr/pb"

func (p *Player) GetTbGender() pb.Gender {
	switch p.Gender {
	case "Girl":
		return pb.Gender_GenderWoman
	case "Boy":
		return pb.Gender_GenderMan
	default:
		return pb.Gender_GenderWoman
	}
}

func (p *Player) GetCurLineup() *pb.LineupInfo {
	var avList []*pb.LineupAvatar
	maxEnergy := &pb.SpBarInfo{
		CurSp: 10000,
		MaxSp: 10000,
	}

	for i, id := range p.Lineup {
		avatar := &pb.LineupAvatar{
			Id:         id,
			Hp:         10000,
			Slot:       uint32(i),
			SpBar:      maxEnergy,
			AvatarType: pb.AvatarType_AVATAR_FORMAL_TYPE,
		}
		avList = append(avList, avatar)
	}

	return &pb.LineupInfo{
		Name:       "QingqueSR",
		AvatarList: avList,
	}
}

func (p *Player) GetAvatars() []*pb.Avatar {
	var avatars []*pb.Avatar

	for _, id := range p.AvatarList {
		avatar := &pb.Avatar{
			BaseAvatarId: id,
			Promotion:    6,
			Rank:         6,
			Level:        80,
		}
		avatars = append(avatars, avatar)
	}

	return avatars
}

func (p *Player) GetMarchMultiPathType() pb.MultiPathAvatarType {
	switch p.MarchType {
	case "Imaginary":
		return pb.MultiPathAvatarType_Mar_7thRogueType
	case "Ice":
		return pb.MultiPathAvatarType_Mar_7thKnightType
	default:
		return pb.MultiPathAvatarType_MultiPathAvatarTypeNone
	}
}

func (p *Player) GetTbMultiPathType() pb.MultiPathAvatarType {
	switch p.Gender {
	case "Girl":
		switch p.TbType {
		case "Imaginary":
			return pb.MultiPathAvatarType_GirlShamanType
		case "Fire":
			return pb.MultiPathAvatarType_GirlKnightType
		case "Physical":
			return pb.MultiPathAvatarType_GirlWarriorType
		case "Ice":
			return pb.MultiPathAvatarType_GirlMemoryType
		default:
			return pb.MultiPathAvatarType_MultiPathAvatarTypeNone
		}
	case "Boy":
		switch p.TbType {
		case "Imaginary":
			return pb.MultiPathAvatarType_BoyShamanType
		case "Fire":
			return pb.MultiPathAvatarType_BoyKnightType
		case "Physical":
			return pb.MultiPathAvatarType_BoyWarriorType
		case "Ice":
			return pb.MultiPathAvatarType_BoyMemoryType
		default:
			return pb.MultiPathAvatarType_MultiPathAvatarTypeNone
		}
	default:
		return pb.MultiPathAvatarType_MultiPathAvatarTypeNone
	}
}

func createDefault() Player {
	return Player{
		Uid:        1,
		Username:   "yulian",
		Gender:     "Boy",
		MarchType:  "Imaginary",
		TbType:     "Ice",
		AvatarList: []uint32{1201, 1001, 8001},
		Lineup:     []uint32{1201, 1001, 8001},
	}
}
