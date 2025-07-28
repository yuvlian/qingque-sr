package gameserver

import (
	"fmt"
	"strconv"

	"github.com/yuvlian/qingque-sr/config/ports"
	"github.com/yuvlian/qingque-sr/gameserver/connection"
)

func Start() {
	addr := "127.0.0.1:" + strconv.Itoa(int(ports.Loaded.GameServer))
	if err := connection.ListenTcp(addr); err != nil {
		fmt.Println(err)
	}
}
