package ports

import "github.com/yuvlian/qingque-sr/config"

type Ports struct {
	GameServer uint16
	SdkServer  uint16
}

func Load(filePath string) (Ports, error) {
	return config.LoadConfigFromFile[Ports](filePath)
}

func (h Ports) Save(filePath string) error {
	return config.SaveConfigToFile(filePath, h)
}

var Loaded = createDefault()

func Init(filePath string) error {
	return config.InitLoaded(filePath, &Loaded)
}
