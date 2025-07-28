package player

import (
	"time"

	"github.com/yuvlian/qingque-sr/config"
)

type Player struct {
	Uid        uint32
	Username   string
	Gender     string
	MarchType  string
	TbType     string
	AvatarList []uint32
	Lineup     []uint32
}

func Load(filePath string) (Player, error) {
	return config.LoadConfigFromFile[Player](filePath)
}

func (h Player) Save(filePath string) error {
	return config.SaveConfigToFile(filePath, h)
}

var Loaded = createDefault()
var lastModTime time.Time

func Init(filePath string) error {
	return config.InitLoadedWithTime(filePath, &lastModTime, &Loaded)
}

func Reload(filePath string) (bool, error) {
	return config.ReloadLoaded(filePath, &lastModTime, &Loaded)
}

func Update(filePath string, newData Player) error {
	Loaded = newData
	err := Loaded.Save(filePath)
	if err != nil {
		return err
	}
	return nil
}
