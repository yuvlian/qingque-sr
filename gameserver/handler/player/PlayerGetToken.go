package player

import (
	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func PlayerGetToken(s *session.Session) error {
	rsp := &pb.PlayerGetTokenScRsp{
		Uid: player.Loaded.Uid,
	}
	return s.Send(cid.PlayerGetTokenScRsp, rsp)
}
