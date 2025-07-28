package scene

import "github.com/yuvlian/qingque-sr/config"

type Scene struct {
	PlaneID uint32
	Player  PlayerSceneInfo
	Calyx   CalyxSceneInfo
}

type Vector3 struct {
	X int32
	Y int32
	Z int32
}

type PlayerSceneInfo struct {
	Pos          Vector3
	BaseAvatarId uint32
	MapLayer     uint32
}

type CalyxSceneInfo struct {
	Pos      Vector3
	GroupID  uint32
	InstID   uint32
	EntityID uint32
	PropID   uint32
}

func Load(filePath string) (Scene, error) {
	return config.LoadConfigFromFile[Scene](filePath)
}

func (s Scene) Save(filePath string) error {
	return config.SaveConfigToFile(filePath, s)
}

var Loaded = createDefault()

func Init(filePath string) error {
	return config.InitLoaded(filePath, &Loaded)
}
