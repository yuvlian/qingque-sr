package main

import (
	"fmt"

	"github.com/yuvlian/qingque-sr/config"
	"github.com/yuvlian/qingque-sr/config/bot"
	"github.com/yuvlian/qingque-sr/config/hotfix"
	"github.com/yuvlian/qingque-sr/config/lx"
	"github.com/yuvlian/qingque-sr/config/player"
	"github.com/yuvlian/qingque-sr/config/ports"
	"github.com/yuvlian/qingque-sr/config/scene"
	"github.com/yuvlian/qingque-sr/config/srtools"
	"github.com/yuvlian/qingque-sr/gameserver"
	"github.com/yuvlian/qingque-sr/sdkserver"
)

func initOrWarn(name string, fn func(string) error, path string) {
	fmt.Println("initializing " + path + "...")
	if err := fn(path); err != nil {
		fmt.Printf("failed to init %s (%s): %v\n", name, path, err)
		fmt.Printf("creating a default file for %s\n", name)
	}
}

func main() {
	fmt.Print("!! https://github.com/yuvlian/qingque-sr !!\n\n")
	initOrWarn("Player", player.Init, config.PlayerFilePath)
	initOrWarn("Bot", bot.Init, config.BotFilePath)
	bot.AssertNotEqualUid(player.Loaded.Uid)
	bot.InitCommandRunner()
	initOrWarn("Hotfix", hotfix.Init, config.HotfixFilePath)
	initOrWarn("Ports", ports.Init, config.PortsFilePath)
	initOrWarn("SRToolsLite", srtools.Init, config.SRToolsLiteFilePath)
	initOrWarn("Scene", scene.Init, config.SceneFilePath)
	initOrWarn("Lx", lx.Init, config.LxFilePath)
	fmt.Print("\n")
	go sdkserver.Start()
	gameserver.Start()
}
