package dispatch

import (
	"encoding/base64"
	"net/http"
	"strconv"

	"github.com/yuvlian/qingque-sr/config/hotfix"
	"github.com/yuvlian/qingque-sr/config/ports"
	"github.com/yuvlian/qingque-sr/pb"
	"google.golang.org/protobuf/proto"
)

func AddHandlers(h *http.ServeMux) {
	h.HandleFunc("/query_dispatch", queryDispatchHandler)
	h.HandleFunc("/query_gateway", queryGatewayHandler)
}

func queryDispatchHandler(w http.ResponseWriter, r *http.Request) {
	dispatch := &pb.Dispatch{
		Retcode: 0,
		Msg:     "OK",
		RegionList: []*pb.RegionInfo{
			{
				Name:        "qingque",
				DisplayName: "qingque",
				Title:       "qingque",
				EnvType:     "2",
				Msg:         "OK",
				DispatchUrl: "http://127.0.0.1:" + strconv.Itoa(int(ports.Loaded.SdkServer)) + "/query_gateway",
			},
		},
		TopSeverRegionName: "qingque",
	}

	encoded, err := proto.Marshal(dispatch)
	if err != nil {
		panic(err)
	}

	b64string := base64.StdEncoding.EncodeToString(encoded)
	w.Header().Set("Content-Type", "text/plain")
	w.Write([]byte(b64string))
}

func queryGatewayHandler(w http.ResponseWriter, r *http.Request) {
	gateServer := &pb.GateServer{
		LuaUrl:                        "",
		MdkResVersion:                 "",
		ExResourceUrl:                 "",
		AssetBundleUrl:                "",
		IfixUrl:                       "",
		IfixVersion:                   "0",
		UseTcp:                        true,
		Ip:                            "127.0.0.1",
		Port:                          uint32(ports.Loaded.GameServer),
		EnableVersionUpdate:           true,
		EnableDesignDataVersionUpdate: true,
		EnableSaveReplayFile:          true,
		EnableUploadBattleLog:         true,
		EnableWatermark:               true,
		EventTrackingOpen:             true,
	}

	query := r.URL.Query()
	if version := query.Get("version"); version != "" {
		dispatchSeed := query.Get("dispatch_seed")
		if val, ok := hotfix.TryGetOrFetchHotfix(version, dispatchSeed); ok {
			gateServer.LuaUrl = val.LuaUrl
			gateServer.MdkResVersion = val.LuaVersion
			gateServer.ExResourceUrl = val.ExResourceUrl
			gateServer.AssetBundleUrl = val.AssetBundleUrl
			gateServer.IfixUrl = val.IfixUrl
		}
	}

	encoded, err := proto.Marshal(gateServer)
	if err != nil {
		panic(err)
	}

	b64string := base64.StdEncoding.EncodeToString(encoded)
	w.Header().Set("Content-Type", "text/plain")
	w.Write([]byte(b64string))
}
