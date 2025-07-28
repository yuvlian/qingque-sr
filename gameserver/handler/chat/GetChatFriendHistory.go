package chat

import (
	"github.com/yuvlian/qingque-sr/config/bot"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetChatFriendHistory(s *session.Session) error {
	rsp := &pb.GetChatFriendHistoryScRsp{
		FriendHistoryInfo: []*pb.FriendHistoryInfo{
			&pb.FriendHistoryInfo{
				ContactSide:  bot.Loaded.Uid,
				LastSendTime: bot.LastOnlineTime.Unix(),
			},
		},
	}
	return s.Send(cid.GetChatFriendHistoryScRsp, rsp)
}
