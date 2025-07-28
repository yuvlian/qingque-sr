package hotfix

import "github.com/yuvlian/qingque-sr/config"

type HotfixValue struct {
	AssetBundleUrl string
	ExResourceUrl  string
	IfixUrl        string
	LuaUrl         string
	LuaVersion     string
}

type Hotfix map[string]HotfixValue

func Load(filePath string) (Hotfix, error) {
	return config.LoadConfigFromFile[Hotfix](filePath)
}

func (h Hotfix) Save(filePath string) error {
	return config.SaveConfigToFile(filePath, h)
}

var Loaded = createDefault()

func Init(filePath string) error {
	return config.InitLoaded(filePath, &Loaded)
}

func Update(filePath, key string, value HotfixValue) error {
	Loaded[key] = value
	return Loaded.Save(filePath)
}
