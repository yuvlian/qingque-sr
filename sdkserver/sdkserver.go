package sdkserver

import (
	"net/http"
	"strconv"

	"github.com/yuvlian/qingque-sr/config/ports"
	"github.com/yuvlian/qingque-sr/sdkserver/auth"
	"github.com/yuvlian/qingque-sr/sdkserver/dispatch"
	"github.com/yuvlian/qingque-sr/sdkserver/extra"
	"github.com/yuvlian/qingque-sr/sdkserver/logger"
)

func Start() {
	mux := http.NewServeMux()
	auth.AddHandlers(mux)
	dispatch.AddHandlers(mux)
	extra.AddHandlers(mux)

	addr := "127.0.0.1:" + strconv.Itoa(int(ports.Loaded.SdkServer))
	http.ListenAndServe(addr, logger.Middleware(mux, addr))
}
