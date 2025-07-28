package chat

import (
	"github.com/yuvlian/qingque-sr/config/bot"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetPrivateChatHistory(s *session.Session) error {
	var req pb.GetPrivateChatHistoryCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	chatHistory := bot.GetChatHistory()
	createTime := uint64(bot.LastOnlineTime.UnixMilli())
	messages := make([]*pb.ChatMessageData, len(chatHistory))
	for i, msg := range chatHistory {
		messages[i] = &pb.ChatMessageData{
			MessageType: pb.MsgType_MSG_TYPE_CUSTOM_TEXT,
			CreateTime:  createTime + uint64(i),
			Content:     msg,
			SenderId:    bot.Loaded.Uid,
		}
	}

	rsp := &pb.GetPrivateChatHistoryScRsp{
		ContactSide:     req.ContactSide,
		TargetSide:      bot.Loaded.Uid,
		ChatMessageList: messages,
	}
	return s.Send(cid.GetPrivateChatHistoryScRsp, rsp)
}
