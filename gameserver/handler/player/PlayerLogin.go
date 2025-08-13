package player

import (
	"time"

	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func PlayerLogin(s *session.Session) error {
	rsp := &pb.PlayerLoginScRsp{
		BasicInfo: &pb.PlayerBasicInfo{
			Nickname:   player.Loaded.Username,
			Level:      61,
			Stamina:    240,
			WorldLevel: 5,
		},
		ServerTimestampMs: uint64(time.Now().UnixMilli()),
		Stamina:           240,
	}
	return s.Send(cid.PlayerLoginScRsp, rsp)
}
