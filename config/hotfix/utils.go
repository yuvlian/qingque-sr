package hotfix

import (
	"encoding/base64"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/yuvlian/qingque-sr/config"
	"github.com/yuvlian/qingque-sr/pb"
	"google.golang.org/protobuf/proto"
)

const (
	ProxyHost  = "proxy1.neonteam.dev"
	CNProdHost = "prod-gf-cn-dp01.bhsr.com"
	CNBetaHost = "beta-release01-cn.bhsr.com"
	OSProdHost = "prod-official-asia-dp01.starrails.com"
	OSBetaHost = "beta-release01-asia.starrails.com"
)

func getHostFromVersion(version string) (string, error) {
	switch {
	case strings.HasPrefix(version, "CNPROD"):
		return CNProdHost, nil
	case strings.HasPrefix(version, "CNBETA"):
		return CNBetaHost, nil
	case strings.HasPrefix(version, "OSPROD"):
		return OSProdHost, nil
	case strings.HasPrefix(version, "OSBETA"):
		return OSBetaHost, nil
	default:
		return "", fmt.Errorf("invalid version: %s", version)
	}
}

func fetchHotfixFromOfficial(version, dispatchSeed string) (HotfixValue, error) {
	host, err := getHostFromVersion(version)
	if err != nil {
		return HotfixValue{}, fmt.Errorf("invalid version: %w", err)
	}

	target := fmt.Sprintf(
		"https://%s/%s/query_gateway?version=%s&dispatch_seed=%s&language_type=1&platform_type=2&channel_id=1&sub_channel_id=1&is_need_url=1&account_type=1",
		ProxyHost, host, version, dispatchSeed,
	)

	resp, err := http.Get(target)
	if err != nil {
		return HotfixValue{}, fmt.Errorf("http get failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return HotfixValue{}, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return HotfixValue{}, fmt.Errorf("failed reading body: %w", err)
	}

	decoded, err := base64.StdEncoding.DecodeString(string(body))
	if err != nil {
		return HotfixValue{}, fmt.Errorf("base64 decode failed: %w", err)
	}

	var gateServer pb.GateServer
	if err := proto.Unmarshal(decoded, &gateServer); err != nil {
		return HotfixValue{}, fmt.Errorf("proto unmarshal failed: %w", err)
	}

	if gateServer.AssetBundleUrl == "" && gateServer.ExResourceUrl == "" {
		return HotfixValue{}, fmt.Errorf("asset bundle and ex resource url are both empty.")
	}

	return HotfixValue{
		LuaUrl:         gateServer.LuaUrl,
		LuaVersion:     gateServer.MdkResVersion,
		ExResourceUrl:  gateServer.ExResourceUrl,
		AssetBundleUrl: gateServer.AssetBundleUrl,
		IfixUrl:        gateServer.IfixUrl,
	}, nil
}

func TryGetOrFetchHotfix(version, dispatchSeed string) (HotfixValue, bool) {
	if val, ok := Loaded[version]; ok {
		return val, true
	}

	fmt.Printf("hotfix for %s not found. starting auto hotfix.\n", version)
	if dispatchSeed == "" {
		fmt.Println("missing 'dispatch_seed'. cannot auto hotfix.")
		return HotfixValue{}, false
	}

	newHfVal, err := fetchHotfixFromOfficial(version, dispatchSeed)
	if err != nil {
		fmt.Println("auto hotfix failed:", err)
		return HotfixValue{}, false
	}

	if err := Update(config.HotfixFilePath, version, newHfVal); err != nil {
		fmt.Println("failed to save updated hotfix:", err)
	} else {
		fmt.Println("auto hotfix success.")
	}

	return newHfVal, true
}

func createDefault() Hotfix {
	return Hotfix{}
}
