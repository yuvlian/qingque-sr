package bot

import (
	_ "errors"
	"fmt"

	_ "github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func handleHelp(s *session.Session, cmd Command, notify *pb.RevcMsgScNotify) error {
	if len(cmd.Args) == 0 {
		meta, _ := commands[cmd.Name]
		notify.MessageText = meta.Header
		if err := s.Send(cid.RevcMsgScNotify, notify); err != nil {
			return err
		}
		notify.MessageText = meta.Description
		return s.Send(cid.RevcMsgScNotify, notify)
	}

	if meta, ok := commands[cmd.Args[0]]; ok {
		notify.MessageText = meta.Header
		if err := s.Send(cid.RevcMsgScNotify, notify); err != nil {
			return err
		}
		notify.MessageText = meta.Description
		return s.Send(cid.RevcMsgScNotify, notify)
	}

	notify.MessageText = fmt.Sprintf("Couldn't find command '%s'", cmd.Args[0])
	return s.Send(cid.RevcMsgScNotify, notify)
}

func handleMarch(s *session.Session, cmd Command, notify *pb.RevcMsgScNotify) error {
	notify.MessageText = fmt.Sprintf("March command executed with args: %v", cmd.Args)
	return s.Send(cid.RevcMsgScNotify, notify)
}

func handleTb(s *session.Session, cmd Command, notify *pb.RevcMsgScNotify) error {
	notify.MessageText = fmt.Sprintf("TB command executed with args: %v", cmd.Args)
	return s.Send(cid.RevcMsgScNotify, notify)
}

func handleGender(s *session.Session, cmd Command, notify *pb.RevcMsgScNotify) error {
	notify.MessageText = fmt.Sprintf("Gender command executed with args: %v", cmd.Args)
	return s.Send(cid.RevcMsgScNotify, notify)
}

func InitCommandRunner() {
	setExec := func(name string, exec func(*session.Session, Command, *pb.RevcMsgScNotify) error) {
		meta := commands[name]
		meta.Execute = exec
		commands[name] = meta
	}
	setExec("help", handleHelp)
	setExec("march", handleMarch)
	setExec("tb", handleTb)
	setExec("gender", handleGender)
	commandNames = concatCommandNames()
	cmdsInitialized = true
}

func HandleCommand(s *session.Session, req *pb.SendMsgCsReq, notify *pb.RevcMsgScNotify) error {
	cmd, err := parseCommand(req.MessageText)
	if err != nil {
		notify.MessageText = err.Error()
		return s.Send(cid.RevcMsgScNotify, notify)
	}

	meta := commands[cmd.Name]
	if cmdsInitialized {
		return meta.Execute(s, cmd, notify)
	}
	return nil
}
