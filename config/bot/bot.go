package bot

import "github.com/yuvlian/qingque-sr/config"

type Bot struct {
	Uid          uint32
	Username     string
	HeadIconID   uint32
	ChatBubbleID uint32
}

func Load(filePath string) (Bot, error) {
	return config.LoadConfigFromFile[Bot](filePath)
}

func (h Bot) Save(filePath string) error {
	return config.SaveConfigToFile(filePath, h)
}

var Loaded = createDefault()

func Init(filePath string) error {
	return config.InitLoaded(filePath, &Loaded)
}
