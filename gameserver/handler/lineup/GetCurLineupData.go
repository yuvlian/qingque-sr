package lineup

import (
	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetCurLineupData(s *session.Session) error {
	rsp := &pb.GetCurLineupDataScRsp{
		Lineup: player.Loaded.GetCurLineup(),
	}
	return s.Send(cid.GetCurLineupDataScRsp, rsp)
}
