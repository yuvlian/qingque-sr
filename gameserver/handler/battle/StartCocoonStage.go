package battle

import (
	"fmt"
	"time"

	"github.com/yuvlian/qingque-sr/config"
	"github.com/yuvlian/qingque-sr/config/srtools"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func StartCocoonStage(s *session.Session) error {
	var req pb.StartCocoonStageCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	wasReloaded, err := srtools.Reload(config.SRToolsLiteFilePath)
	if err != nil {
		fmt.Println("failed to reload SRToolsLite:", err)
	}
	if wasReloaded {
		fmt.Println("reloaded SRToolsLite!")
	} else {
		fmt.Println("using already loaded SRToolsLite.")
	}

	battleAvatars, errList := srtools.Loaded.GetBattleAvatars()
	if len(errList) > 0 {
		for _, err := range errList {
			fmt.Println(err)
		}
	}

	battleInfo := &pb.SceneBattleInfo{
		BuffList:         srtools.Loaded.GetBattleBuffs(),
		BattleId:         srtools.Loaded.GetBattleID(),
		StageId:          srtools.Loaded.GetStageID(),
		RoundsLimit:      srtools.Loaded.GetCycleCount(),
		MonsterWaveList:  srtools.Loaded.GetBattleWaves(),
		BattleAvatarList: battleAvatars,
		LogicRandomSeed:  uint32(time.Now().Unix() / 1000),
	}

	rsp := &pb.StartCocoonStageScRsp{
		CocoonId:     req.CocoonId,
		PropEntityId: req.PropEntityId,
		Wave:         1,
		BattleInfo:   battleInfo,
	}
	return s.Send(cid.StartCocoonStageScRsp, rsp)
}
