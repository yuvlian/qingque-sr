package ports

func createDefault() Ports {
	return Ports{
		GameServer: 23301,
		SdkServer:  21000,
	}
}
