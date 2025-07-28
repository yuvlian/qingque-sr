package connection

import (
	"fmt"
	"net"

	"github.com/yuvlian/qingque-sr/gameserver/handler"
	"github.com/yuvlian/qingque-sr/gameserver/session"
)

func ListenTcp(addr string) error {
	listener, err := net.Listen("tcp", addr)
	if err != nil {
		return err
	}

	defer listener.Close()
	fmt.Println("gameserver listening on:", addr)

	for {
		conn, err := listener.Accept()
		fmt.Println("new client:", conn.RemoteAddr())
		if err != nil {
			return err
		}
		go startSession(conn)
	}
}

func startSession(conn net.Conn) {
	sess := &session.Session{
		Conn: conn,
	}

	defer sess.Conn.Close()
	for {
		if err := sess.Recv(); err != nil {
			fmt.Println("error reading packet:", err)
			break
		}
		if err := sess.HandleWith(handler.SessionHandler); err != nil {
			fmt.Println("error handling session:", err)
			break
		}
	}
}
