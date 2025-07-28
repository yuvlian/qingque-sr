package mission

import (
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

func GetMissionStatus(s *session.Session) error {
	var req pb.GetMissionStatusCsReq
	if err := s.DecodeReq(&req); err != nil {
		return err
	}

	subMissions := make([]*pb.Mission, 0, len(req.SubMissionIdList))
	for _, id := range req.SubMissionIdList {
		subMissions = append(subMissions, &pb.Mission{
			Id:       id,
			Progress: 1,
			Status:   pb.MissionStatus_MISSION_FINISH,
		})
	}

	rsp := &pb.GetMissionStatusScRsp{
		FinishedMainMissionIdList: req.MainMissionIdList,
		SubMissionStatusList:      subMissions,
	}
	return s.Send(cid.GetMissionStatusScRsp, rsp)
}
