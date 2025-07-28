package chat

import (
	"github.com/yuvlian/qingque-sr/config/bot"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetLoginChatInfo(s *session.Session) error {
	rsp := &pb.GetLoginChatInfoScRsp{
		ContactIdList: []uint32{bot.Loaded.Uid},
	}
	return s.Send(cid.GetLoginChatInfoScRsp, rsp)
}
