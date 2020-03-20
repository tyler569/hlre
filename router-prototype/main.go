package main

import (
	"fmt"

	"github.com/google/gopacket"
	"github.com/google/gopacket/pcap"
)

func handlePacket(packet gopacket.Packet) {
	fmt.Println(packet)
}

func main() {
	// fmt.Println("This is a prototype of the flow router")

	if handle, err := pcap.OpenLive("wlp2s0", 1600, true, pcap.BlockForever); err != nil {
		panic(err)
	} else {
		packetSource := gopacket.NewPacketSource(handle, handle.LinkType())
		for packet := range packetSource.Packets() {
			handlePacket(packet)
		}
	}
}
