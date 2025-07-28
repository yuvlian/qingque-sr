package config

import (
	"encoding/json"
	"os"
	"time"
)

const (
	BotFilePath         string = "bot.json"
	HotfixFilePath      string = "hotfix.json"
	LxFilePath          string = "lx.json"
	PlayerFilePath      string = "player.json"
	PortsFilePath       string = "ports.json"
	SceneFilePath       string = "scene.json"
	SRToolsLiteFilePath string = "config.json"
)

func LoadConfigFromString[T any](input string) (T, error) {
	var result T
	err := json.Unmarshal([]byte(input), &result)
	return result, err
}

func LoadConfigFromFile[T any](filePath string) (T, error) {
	var result T

	data, err := os.ReadFile(filePath)
	if err != nil {
		return result, err
	}

	err = json.Unmarshal(data, &result)
	return result, err
}

func SaveConfigToFile[T any](filePath string, data T) error {
	bytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return err
	}

	return os.WriteFile(filePath, bytes, 0644)
}

func InitLoadedWithTime[T any](filePath string, lastMod *time.Time, target *T) error {
	data, err := LoadConfigFromFile[T](filePath)
	if err != nil {
		if saveErr := SaveConfigToFile(filePath, *target); saveErr != nil {
			return saveErr
		}
		return err
	}

	*target = data

	info, statErr := os.Stat(filePath)
	if statErr == nil {
		*lastMod = info.ModTime()
	}

	return nil
}

func InitLoaded[T any](filePath string, target *T) error {
	data, err := LoadConfigFromFile[T](filePath)
	if err != nil {
		if saveErr := SaveConfigToFile(filePath, *target); saveErr != nil {
			return saveErr
		}
		return err
	}

	*target = data
	return nil
}

func ReloadLoaded[T any](filePath string, lastMod *time.Time, target *T) (bool, error) {
	info, err := os.Stat(filePath)
	if err != nil {
		return false, err
	}
	if !info.ModTime().After(*lastMod) {
		return false, nil
	}

	newData, err := LoadConfigFromFile[T](filePath)
	if err != nil {
		return false, err
	}

	*target = newData
	*lastMod = info.ModTime()
	return true, nil
}
