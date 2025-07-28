package player

import (
	"fmt"
	"time"

	"github.com/yuvlian/qingque-sr/config"
	"github.com/yuvlian/qingque-sr/config/lx"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func PlayerHeartBeat(s *session.Session) error {
	var req pb.PlayerHeartBeatCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	wasReloaded, err := lx.Reload(config.LxFilePath)
	if err != nil {
		fmt.Println("failed to reload Lx:", err)
	}
	if wasReloaded {
		fmt.Println("reloaded Lx.")
	}

	curTimeMs := time.Now().UnixMilli()
	rsp := &pb.PlayerHeartBeatScRsp{
		ClientTimeMs: req.ClientTimeMs,
		ServerTimeMs: uint64(curTimeMs),
	}

	if lx.Loaded.ShouldExec {
		rsp.DownloadData = &pb.ClientDownloadData{
			Time: curTimeMs,
			Data: []byte(lx.Loaded.LuaContent),
		}
		if err := lx.Update(config.LxFilePath, false, ""); err != nil {
			return err
		}
	}

	return s.Send(cid.PlayerHeartBeatScRsp, rsp)
}
