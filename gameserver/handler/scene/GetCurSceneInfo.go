package scene

import (
	"github.com/yuvlian/qingque-sr/config/scene"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetCurSceneInfo(s *session.Session) error {
	rsp := &pb.GetCurSceneInfoScRsp{
		Scene: scene.Loaded.GetSceneInfo(),
	}

	return s.Send(cid.GetCurSceneInfoScRsp, rsp)
}
