package lineup

import (
	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetAllLineupData(s *session.Session) error {
	rsp := &pb.GetAllLineupDataScRsp{
		LineupList: []*pb.LineupInfo{player.Loaded.GetCurLineup()},
	}
	return s.Send(cid.GetAllLineupDataScRsp, rsp)
}
