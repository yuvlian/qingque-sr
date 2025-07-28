package scene

import "github.com/yuvlian/qingque-sr/pb"

func (s *Scene) GetSceneInfo() *pb.SceneInfo {
	return &pb.SceneInfo{
		PlaneId:      s.PlaneID,
		FloorId:      s.PlaneID*1000 + 1,
		EntryId:      s.PlaneID*100 + 1,
		GameModeType: 2,
		EntityGroupList: []*pb.SceneEntityGroupInfo{
			{
				State:   1,
				GroupId: 0,
				EntityList: []*pb.SceneEntityInfo{
					{
						GroupId:  0,
						InstId:   0,
						EntityId: 0,
						Entity: &pb.SceneEntityInfo_Actor{
							Actor: &pb.SceneActorInfo{
								AvatarType:   pb.AvatarType_AVATAR_FORMAL_TYPE,
								BaseAvatarId: s.Player.BaseAvatarId,
								MapLayer:     s.Player.MapLayer,
								Uid:          1,
							},
						},
						Motion: &pb.MotionInfo{
							Pos: &pb.Vector{
								X: s.Player.Pos.X,
								Y: s.Player.Pos.Y,
								Z: s.Player.Pos.Z,
							},
							Rot: &pb.Vector{},
						},
					},
				},
			},
			{
				State:   1,
				GroupId: s.Calyx.GroupID,
				EntityList: []*pb.SceneEntityInfo{
					{
						GroupId:  s.Calyx.GroupID,
						InstId:   s.Calyx.InstID,
						EntityId: s.Calyx.EntityID,
						Entity: &pb.SceneEntityInfo_Prop{
							Prop: &pb.ScenePropInfo{
								PropId:    s.Calyx.PropID,
								PropState: 1,
							},
						},
						Motion: &pb.MotionInfo{
							Pos: &pb.Vector{
								X: s.Calyx.Pos.X,
								Y: s.Calyx.Pos.Y,
								Z: s.Calyx.Pos.Z,
							},
							Rot: &pb.Vector{},
						},
					},
				},
			},
		},
	}
}

func createDefault() Scene {
	return Scene{
		PlaneID: 20313,
		Player: PlayerSceneInfo{
			Pos: Vector3{
				X: 40748,
				Y: 192819,
				Z: 439218,
			},
			BaseAvatarId: 1014,
			MapLayer:     2,
		},
		Calyx: CalyxSceneInfo{
			Pos: Vector3{
				X: 31440,
				Y: 192820,
				Z: 433790,
			},
			GroupID:  186,
			InstID:   300001,
			EntityID: 1337,
			PropID:   808,
		},
	}
}
