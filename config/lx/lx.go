package lx

import (
	"time"

	"github.com/yuvlian/qingque-sr/config"
)

type Lx struct {
	ShouldExec bool
	LuaContent string
}

func Load(filePath string) (Lx, error) {
	return config.LoadConfigFromFile[Lx](filePath)
}

func (l Lx) Save(filePath string) error {
	return config.SaveConfigToFile(filePath, l)
}

var Loaded = Lx{}
var lastModTime time.Time

func Init(filePath string) error {
	return config.InitLoadedWithTime(filePath, &lastModTime, &Loaded)
}

func Reload(filePath string) (bool, error) {
	return config.ReloadLoaded(filePath, &lastModTime, &Loaded)
}

func Update(filePath string, shouldExec bool, luaContent string) error {
	Loaded.ShouldExec = shouldExec
	Loaded.LuaContent = luaContent
	return Loaded.Save(filePath)
}
