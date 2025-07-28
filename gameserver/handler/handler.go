package handler

import (
	"fmt"

	"github.com/yuvlian/qingque-sr/gameserver/handler/avatar"
	"github.com/yuvlian/qingque-sr/gameserver/handler/battle"
	"github.com/yuvlian/qingque-sr/gameserver/handler/chat"
	"github.com/yuvlian/qingque-sr/gameserver/handler/friend"
	"github.com/yuvlian/qingque-sr/gameserver/handler/lineup"
	"github.com/yuvlian/qingque-sr/gameserver/handler/mission"
	"github.com/yuvlian/qingque-sr/gameserver/handler/player"
	"github.com/yuvlian/qingque-sr/gameserver/handler/scene"
	"github.com/yuvlian/qingque-sr/gameserver/session"
	"github.com/yuvlian/qingque-sr/pb/cid"
)

var dummyMap = map[uint16]uint16{
	cid.GetLevelRewardTakenListCsReq:        cid.GetLevelRewardTakenListScRsp,
	cid.GetRogueScoreRewardInfoCsReq:        cid.GetRogueScoreRewardInfoScRsp,
	cid.QueryProductInfoCsReq:               cid.QueryProductInfoScRsp,
	cid.GetQuestDataCsReq:                   cid.GetQuestDataScRsp,
	cid.GetQuestRecordCsReq:                 cid.GetQuestRecordScRsp,
	cid.GetCurAssistCsReq:                   cid.GetCurAssistScRsp,
	cid.GetRogueHandbookDataCsReq:           cid.GetRogueHandbookDataScRsp,
	cid.GetDailyActiveInfoCsReq:             cid.GetDailyActiveInfoScRsp,
	cid.GetFightActivityDataCsReq:           cid.GetFightActivityDataScRsp,
	cid.GetMultipleDropInfoCsReq:            cid.GetMultipleDropInfoScRsp,
	cid.GetPlayerReturnMultiDropInfoCsReq:   cid.GetPlayerReturnMultiDropInfoScRsp,
	cid.GetShareDataCsReq:                   cid.GetShareDataScRsp,
	cid.GetTreasureDungeonActivityDataCsReq: cid.GetTreasureDungeonActivityDataScRsp,
	cid.PlayerReturnInfoQueryCsReq:          cid.PlayerReturnInfoQueryScRsp,
	cid.GetPlayerBoardDataCsReq:             cid.GetPlayerBoardDataScRsp,
	cid.GetActivityScheduleConfigCsReq:      cid.GetActivityScheduleConfigScRsp,
	cid.GetMissionDataCsReq:                 cid.GetMissionDataScRsp,
	cid.GetChallengeCsReq:                   cid.GetChallengeScRsp,
	cid.GetCurChallengeCsReq:                cid.GetCurChallengeScRsp,
	cid.GetRogueInfoCsReq:                   cid.GetRogueInfoScRsp,
	cid.GetExpeditionDataCsReq:              cid.GetExpeditionDataScRsp,
	cid.GetJukeboxDataCsReq:                 cid.GetJukeboxDataScRsp,
	cid.SyncClientResVersionCsReq:           cid.SyncClientResVersionScRsp,
	cid.DailyFirstMeetPamCsReq:              cid.DailyFirstMeetPamScRsp,
	cid.GetMuseumInfoCsReq:                  cid.GetMuseumInfoScRsp,
	cid.GetLoginActivityCsReq:               cid.GetLoginActivityScRsp,
	cid.GetRaidInfoCsReq:                    cid.GetRaidInfoScRsp,
	cid.GetTrialActivityDataCsReq:           cid.GetTrialActivityDataScRsp,
	cid.GetBoxingClubInfoCsReq:              cid.GetBoxingClubInfoScRsp,
	cid.GetNpcStatusCsReq:                   cid.GetNpcStatusScRsp,
	cid.TextJoinQueryCsReq:                  cid.TextJoinQueryScRsp,
	cid.GetSecretKeyInfoCsReq:               cid.GetSecretKeyInfoScRsp,
	cid.GetVideoVersionKeyCsReq:             cid.GetVideoVersionKeyScRsp,
	cid.GetCurBattleInfoCsReq:               cid.GetCurBattleInfoScRsp,
	cid.GetPhoneDataCsReq:                   cid.GetPhoneDataScRsp,
	cid.InteractPropCsReq:                   cid.InteractPropScRsp,
	cid.FinishTalkMissionCsReq:              cid.FinishTalkMissionScRsp,
	cid.GetRechargeGiftInfoCsReq:            cid.GetRechargeGiftInfoScRsp,
	cid.GetBagCsReq:                         cid.GetBagScRsp,
	cid.PlayerLoginFinishCsReq:              cid.PlayerLoginFinishScRsp,
	cid.SceneEntityMoveCsReq:                cid.SceneEntityMoveScRsp,
}

var handlerMap = map[uint16]func(*session.Session) error{
	cid.GetAvatarDataCsReq:         avatar.GetAvatarData,
	cid.PVEBattleResultCsReq:       battle.PveBattleResult,
	cid.StartCocoonStageCsReq:      battle.StartCocoonStage,
	cid.GetChatFriendHistoryCsReq:  chat.GetChatFriendHistory,
	cid.GetLoginChatInfoCsReq:      chat.GetLoginChatInfo,
	cid.GetPrivateChatHistoryCsReq: chat.GetPrivateChatHistory,
	cid.GetFriendListInfoCsReq:     friend.GetFriendListInfo,
	cid.ChangeLineupLeaderCsReq:    lineup.ChangeLineupLeader,
	cid.GetAllLineupDataCsReq:      lineup.GetAllLineupData,
	cid.GetCurLineupDataCsReq:      lineup.GetCurLineupData,
	cid.GetMissionStatusCsReq:      mission.GetMissionStatus,
	cid.PlayerGetTokenCsReq:        player.PlayerGetToken,
	cid.PlayerHeartBeatCsReq:       player.PlayerHeartBeat,
	cid.PlayerLoginCsReq:           player.PlayerLogin,
	cid.GetCurSceneInfoCsReq:       scene.GetCurSceneInfo,
}

func SessionHandler(s *session.Session) error {
	if dummyCmd, ok := dummyMap[s.Packet.Cmd]; ok {
		return s.SendDummy(dummyCmd)
	}

	handler, ok := handlerMap[s.Packet.Cmd]
	if !ok {
		fmt.Printf("unhandled cmd: %d\n", s.Packet.Cmd)
		return nil
	}
	return handler(s)
}
