package chat

import (
	"github.com/yuvlian/qingque-sr/config/bot"
	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func SendMsg(s *session.Session) error {
	var req pb.SendMsgCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	playerUid := player.Loaded.Uid
	botUid := bot.Loaded.Uid

	// player -> bot
	notify := &pb.RevcMsgScNotify{
		ChatType:    req.ChatType,
		MessageType: req.MessageType,
		ExtraId:     req.ExtraId,
		MessageText: req.MessageText,
		HNBEPABNBNG: req.HNBEPABNBNG,
		TargetUid:   botUid,
		SourceUid:   playerUid,
	}

	if err := s.Send(cid.RevcMsgScNotify, notify); err != nil {
		return err
	}
	if err := s.Send(cid.SendMsgScRsp, &pb.SendMsgScRsp{}); err != nil {
		return err
	}

	// bot -> player
	notify.TargetUid = playerUid
	notify.SourceUid = botUid
	return bot.HandleCommand(s, &req, notify)
}
