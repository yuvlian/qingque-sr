package chat

import (
	"fmt"

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

	rsp := &pb.GetPrivateChatHistoryScRsp{
		ContactSide: req.ContactSide,
	}

	if req.ContactSide != bot.Loaded.Uid {
		fmt.Println("got something besides bot's uid?")
		return s.Send(cid.GetPrivateChatHistoryScRsp, rsp)
	}

	chatHistory := bot.GetChatHistory()
	createTime := uint64(bot.LastOnlineTime.UnixMilli())

	rsp.ChatMessageList = make([]*pb.ChatMessageData, len(chatHistory))
	for i, msg := range chatHistory {
		rsp.ChatMessageList[i] = &pb.ChatMessageData{
			MessageType: pb.MsgType_MSG_TYPE_CUSTOM_TEXT,
			CreateTime:  createTime + uint64(i),
			Content:     msg,
			SenderId:    bot.Loaded.Uid,
		}
	}

	return s.Send(cid.GetPrivateChatHistoryScRsp, rsp)
}
