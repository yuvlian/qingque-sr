package lineup

import (
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func ChangeLineupLeader(s *session.Session) error {
	var req pb.ChangeLineupLeaderCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	rsp := &pb.ChangeLineupLeaderScRsp{
		Slot: req.Slot,
	}
	return s.Send(cid.ChangeLineupLeaderScRsp, rsp)
}
