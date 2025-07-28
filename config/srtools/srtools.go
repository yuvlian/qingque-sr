package srtools

import (
	"time"

	"github.com/yuvlian/qingque-sr/config"
)

type SRToolsLite struct {
	AvatarConfig []AvatarConfig `json:"avatar_config"`
	BattleConfig BattleConfig   `json:"battle_config"`
}

type AvatarConfig struct {
	ID         uint32    `json:"id"`
	HP         uint32    `json:"hp"`
	SP         uint32    `json:"sp"`
	Level      uint32    `json:"level"`
	Promotion  uint32    `json:"promotion"`
	Rank       uint32    `json:"rank"`
	LightCone  LightCone `json:"lightcone"`
	Relics     []string  `json:"relics"`
	BuffIDList []uint32  `json:"buff_id_list"`
}

type LightCone struct {
	ID        uint32 `json:"id"`
	Rank      uint32 `json:"rank"`
	Level     uint32 `json:"level"`
	Promotion uint32 `json:"promotion"`
}

type BattleConfig struct {
	BattleID     uint32     `json:"battle_id"`
	StageID      uint32     `json:"stage_id"`
	CycleCount   uint32     `json:"cycle_count"`
	MonsterWave  [][]uint32 `json:"monster_wave"`
	MonsterLevel uint32     `json:"monster_level"`
}

func Load(filePath string) (SRToolsLite, error) {
	return config.LoadConfigFromFile[SRToolsLite](filePath)
}

func LoadFromString(json string) (SRToolsLite, error) {
	return config.LoadConfigFromString[SRToolsLite](json)
}

func (h SRToolsLite) Save(filePath string) error {
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
