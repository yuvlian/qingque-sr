package packet

import (
	"encoding/binary"
	"errors"
	"io"
	"net"

	"google.golang.org/protobuf/proto"
)

var (
	headMagic = []byte{0x9D, 0x74, 0xC7, 0x14}
	tailMagic = []byte{0xD7, 0xA1, 0x52, 0xC8}
)

type Packet struct {
	Cmd  uint16
	Head []byte
	Body []byte
}

func ReadPacket(conn net.Conn) (*Packet, error) {
	header := make([]byte, 12)
	if _, err := io.ReadFull(conn, header); err != nil {
		return nil, err
	}

	if !(header[0] == headMagic[0] &&
		header[1] == headMagic[1] &&
		header[2] == headMagic[2] &&
		header[3] == headMagic[3]) {
		return nil, errors.New("invalid head magic")
	}

	cmd := binary.BigEndian.Uint16(header[4:6])
	headLen := binary.BigEndian.Uint16(header[6:8])
	bodyLen := binary.BigEndian.Uint32(header[8:12])

	head := make([]byte, headLen)
	if _, err := io.ReadFull(conn, head); err != nil {
		return nil, err
	}

	body := make([]byte, bodyLen)
	if _, err := io.ReadFull(conn, body); err != nil {
		return nil, err
	}

	tail := make([]byte, 4)
	if _, err := io.ReadFull(conn, tail); err != nil {
		return nil, err
	}
	if !(tail[0] == tailMagic[0] &&
		tail[1] == tailMagic[1] &&
		tail[2] == tailMagic[2] &&
		tail[3] == tailMagic[3]) {
		return nil, errors.New("invalid tail magic")
	}

	return &Packet{
		Cmd:  cmd,
		Head: head,
		Body: body,
	}, nil
}

func (p *Packet) Encode() ([]byte, error) {
	cmd, head, body := p.Cmd, p.Head, p.Body

	headLen := uint16(len(head))
	bodyLen := uint32(len(body))

	totalLen := 4 + 2 + 2 + 4 + len(head) + len(body) + 4
	packet := make([]byte, totalLen)

	offset := 0
	copy(packet[offset:], headMagic)
	offset += 4

	binary.BigEndian.PutUint16(packet[offset:], cmd)
	offset += 2

	binary.BigEndian.PutUint16(packet[offset:], headLen)
	offset += 2

	binary.BigEndian.PutUint32(packet[offset:], bodyLen)
	offset += 4

	copy(packet[offset:], head)
	offset += len(head)

	copy(packet[offset:], body)
	offset += len(body)

	copy(packet[offset:], tailMagic)

	return packet, nil
}

func QuickEncode(cmd uint16, msg proto.Message) ([]byte, error) {
	body, err := proto.Marshal(msg)
	if err != nil {
		return nil, err
	}
	packet := &Packet{
		Cmd:  cmd,
		Head: []byte{},
		Body: body,
	}
	return packet.Encode()
}
