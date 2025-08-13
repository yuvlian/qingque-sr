package friend

import (
	"github.com/yuvlian/qingque-sr/config/bot"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetFriendListInfo(s *session.Session) error {
	createTime := bot.LastOnlineTime.Unix()
	botPlayerInfo := &pb.PlayerSimpleInfo{
		LastActiveTime: createTime,
		HeadIcon:       bot.Loaded.HeadIconID,
		Uid:            bot.Loaded.Uid,
		Nickname:       bot.Loaded.Username,
		ChatBubbleId:   bot.Loaded.ChatBubbleID,
		OnlineStatus:   pb.FriendOnlineStatus_FRIEND_ONLINE_STATUS_ONLINE,
		Level:          40,
		Signature:      "github.com/yuvlian/qingque-sr",
	}
	rsp := &pb.GetFriendListInfoScRsp{
		FriendList: []*pb.FriendSimpleInfo{
			&pb.FriendSimpleInfo{
				PlayerInfo: botPlayerInfo,
				CreateTime: createTime,
				IsMarked:   true,
			},
		},
	}
	return s.Send(cid.GetFriendListInfoScRsp, rsp)
}
