package avatar

import (
	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetAvatarData(s *session.Session) error {
	rsp := &pb.GetAvatarDataScRsp{
		IsGetAll: true,
		CurAvatarPath: map[uint32]pb.MultiPathAvatarType{
			8001: player.Loaded.GetTbMultiPathType(),
			1001: player.Loaded.GetMarchMultiPathType(),
		},
		AvatarList: player.Loaded.GetAvatars(),
	}
	return s.Send(cid.GetAvatarDataScRsp, rsp)
}
