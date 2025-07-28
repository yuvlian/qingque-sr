package battle

import (
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func PveBattleResult(s *session.Session) error {
	var req pb.PVEBattleResultCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	rsp := &pb.PVEBattleResultScRsp{
		EndStatus: req.EndStatus,
		StageId:   req.StageId,
		BattleId:  req.BattleId,
	}
	return s.Send(cid.PVEBattleResultScRsp, rsp)
}
