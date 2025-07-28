package session

import (
	"errors"
	"fmt"
	"net"

	"github.com/yuvlian/qingque-sr/gameserver/packet"
	"google.golang.org/protobuf/proto"
)

type Session struct {
	Conn   net.Conn
	Packet *packet.Packet
}

func (s *Session) Recv() error {
	pk, err := packet.ReadPacket(s.Conn)
	if err != nil {
		return err
	}
	s.Packet = pk
	return nil
}

func (s *Session) SendDummy(cmd uint16) error {
	packet := &packet.Packet{
		Cmd:  cmd,
		Head: []byte{},
		Body: []byte{},
	}
	data, err := packet.Encode()
	if err != nil {
		return err
	}
	_, err = s.Conn.Write(data)
	return err
}

func (s *Session) Send(cmd uint16, msg proto.Message) error {
	data, err := packet.QuickEncode(cmd, msg)
	if err != nil {
		return err
	}
	_, err = s.Conn.Write(data)
	return err
}

func (s *Session) DecodeReq(msg proto.Message) error {
	if s.Packet == nil {
		return errors.New("no packet to decode")
	}
	if err := proto.Unmarshal(s.Packet.Body, msg); err != nil {
		return fmt.Errorf("proto unmarshal failed: %w", err)
	}
	return nil
}

func (s *Session) HandleWith(handler func(*Session) error) error {
	return handler(s)
}
